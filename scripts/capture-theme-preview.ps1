$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type @"
using System;
using System.Runtime.InteropServices;
using System.Text;

public static class FeatherMarkThemeCapture {
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern bool EnumWindows(EnumWindowsProc callback, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint processId);

    [DllImport("user32.dll")]
    public static extern bool IsWindowVisible(IntPtr hWnd);

    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int maxCount);

    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out Rect rect);

    [DllImport("user32.dll")]
    public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBlt, uint flags);

    [StructLayout(LayoutKind.Sequential)]
    public struct Rect {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }
}
"@

$projectRoot = Split-Path -Parent $PSScriptRoot
$configuration = Get-Content -LiteralPath (Join-Path $projectRoot 'src-tauri\tauri.conf.json') -Raw | ConvertFrom-Json
$executable = Join-Path $projectRoot "dist\windows\FeatherMark-$($configuration.version)-windows-x64-portable.exe"
$fixture = Join-Path $projectRoot 'fixtures\sample.md'
$qaDirectory = Join-Path $projectRoot 'qa'
New-Item -ItemType Directory -Path $qaDirectory -Force | Out-Null

function Save-WindowScreenshot([IntPtr]$Window, [string]$Path) {
    $rect = [FeatherMarkThemeCapture+Rect]::new()
    if (-not [FeatherMarkThemeCapture]::GetWindowRect($Window, [ref]$rect)) {
        throw 'Could not read FeatherMark window bounds.'
    }
    $width = $rect.Right - $rect.Left
    $height = $rect.Bottom - $rect.Top
    $bitmap = [Drawing.Bitmap]::new($width, $height)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    $deviceContext = [IntPtr]::Zero
    try {
        $deviceContext = $graphics.GetHdc()
        if (-not [FeatherMarkThemeCapture]::PrintWindow($Window, $deviceContext, 2)) {
            throw 'Windows could not render the FeatherMark window for capture.'
        }
        $graphics.ReleaseHdc($deviceContext)
        $deviceContext = [IntPtr]::Zero
        $bitmap.Save($Path, [Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        if ($deviceContext -ne [IntPtr]::Zero) {
            $graphics.ReleaseHdc($deviceContext)
        }
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

function Get-LargestVisibleWindow([uint32]$ProcessId) {
    $script:bestWindow = [IntPtr]::Zero
    $script:bestArea = 0
    [FeatherMarkThemeCapture]::EnumWindows({
        param([IntPtr]$window, [IntPtr]$expectedProcessId)
        $windowProcessId = 0
        [void][FeatherMarkThemeCapture]::GetWindowThreadProcessId($window, [ref]$windowProcessId)
        $titleBuffer = [Text.StringBuilder]::new(512)
        [void][FeatherMarkThemeCapture]::GetWindowText($window, $titleBuffer, $titleBuffer.Capacity)
        $title = $titleBuffer.ToString()
        if ($windowProcessId -eq $expectedProcessId.ToInt32() -and
            $title -like '*FeatherMark' -and
            [FeatherMarkThemeCapture]::IsWindowVisible($window)) {
            $rect = [FeatherMarkThemeCapture+Rect]::new()
            if ([FeatherMarkThemeCapture]::GetWindowRect($window, [ref]$rect)) {
                $area = ($rect.Right - $rect.Left) * ($rect.Bottom - $rect.Top)
                if ($area -gt $script:bestArea) {
                    $script:bestArea = $area
                    $script:bestWindow = $window
                }
            }
        }
        return $true
    }, [IntPtr]$ProcessId) | Out-Null
    return $script:bestWindow
}

if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
    throw 'Build and package the portable release first.'
}

$process = Start-Process -FilePath $executable -ArgumentList "`"$fixture`"" -PassThru
try {
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    $window = [IntPtr]::Zero
    while ($window -eq [IntPtr]::Zero -and [DateTime]::UtcNow -lt $deadline) {
        Start-Sleep -Milliseconds 200
        $process.Refresh()
        $window = Get-LargestVisibleWindow ([uint32]$process.Id)
    }
    if ($window -eq [IntPtr]::Zero) {
        throw 'FeatherMark did not expose a window in time.'
    }

    [void][FeatherMarkThemeCapture]::SetForegroundWindow($window)
    Start-Sleep -Milliseconds 500
    $first = Join-Path $qaDirectory 'themes-dropdown-before-toggle.png'
    Save-WindowScreenshot $window $first

    [Windows.Forms.SendKeys]::SendWait('^d')
    Start-Sleep -Milliseconds 700
    $second = Join-Path $qaDirectory 'themes-dropdown-after-toggle.png'
    Save-WindowScreenshot $window $second

    Write-Output "BeforeToggle: $first"
    Write-Output "AfterToggle: $second"
}
finally {
    if (-not $process.HasExited) {
        [void]$process.CloseMainWindow()
        if (-not $process.WaitForExit(5000)) {
            $process.Kill()
        }
    }
}
