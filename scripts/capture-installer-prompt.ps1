$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;
using System.Text;

public static class FeatherMarkInstallerUi {
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern bool EnumWindows(EnumWindowsProc callback, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern bool EnumChildWindows(IntPtr parent, EnumWindowsProc callback, IntPtr lParam);

    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int maxCount);

    [DllImport("user32.dll")]
    public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint processId);

    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr hWnd, int id);

    [DllImport("user32.dll")]
    public static extern bool IsWindowEnabled(IntPtr hWnd);

    [DllImport("user32.dll")]
    public static extern IntPtr SendMessage(IntPtr hWnd, uint message, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out Rect rect);

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
$configurationPath = Join-Path $projectRoot "src-tauri\tauri.conf.json"
$version = (Get-Content -LiteralPath $configurationPath -Raw | ConvertFrom-Json).version
$installer = Join-Path $projectRoot "src-tauri\target\release\bundle\nsis\FeatherMark_${version}_x64-setup.exe"
$qaDirectory = Join-Path $projectRoot "qa"
New-Item -ItemType Directory -Path $qaDirectory -Force | Out-Null
$screenshot = Join-Path $qaDirectory "installer-default-app-prompt.png"
$message = 0x00F5 # BM_CLICK
$yesId = 6
$noId = 7
$okId = 1

function Get-WindowText([IntPtr]$Window) {
    $buffer = [Text.StringBuilder]::new(2048)
    [void][FeatherMarkInstallerUi]::GetWindowText($Window, $buffer, $buffer.Capacity)
    return $buffer.ToString()
}

function Get-ProcessWindows([uint32]$ProcessId) {
    $windows = [Collections.Generic.List[IntPtr]]::new()
    [FeatherMarkInstallerUi]::EnumWindows({
        param([IntPtr]$window, [IntPtr]$unused)
        $windowProcessId = 0
        [void][FeatherMarkInstallerUi]::GetWindowThreadProcessId($window, [ref]$windowProcessId)
        if ($windowProcessId -eq $ProcessId) {
            $windows.Add($window)
        }
        return $true
    }, [IntPtr]::Zero) | Out-Null
    return $windows
}

function Get-DescendantText([IntPtr]$Window) {
    $parts = [Collections.Generic.List[string]]::new()
    [FeatherMarkInstallerUi]::EnumChildWindows($Window, {
        param([IntPtr]$child, [IntPtr]$unused)
        $text = Get-WindowText $child
        if ($text) {
            $parts.Add($text)
        }
        return $true
    }, [IntPtr]::Zero) | Out-Null
    return ($parts -join "`n")
}

function Save-WindowScreenshot([IntPtr]$Window, [string]$Path) {
    $rect = [FeatherMarkInstallerUi+Rect]::new()
    if (-not [FeatherMarkInstallerUi]::GetWindowRect($Window, [ref]$rect)) {
        throw "Could not read installer prompt bounds."
    }

    $width = $rect.Right - $rect.Left
    $height = $rect.Bottom - $rect.Top
    $bitmap = [Drawing.Bitmap]::new($width, $height)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, [Drawing.Size]::new($width, $height))
        $bitmap.Save($Path, [Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

$process = Start-Process -FilePath $installer -PassThru
$deadline = [DateTime]::UtcNow.AddSeconds(45)
$captured = $false

while (-not $process.HasExited -and [DateTime]::UtcNow -lt $deadline) {
    $windows = Get-ProcessWindows ([uint32]$process.Id)

    foreach ($window in $windows) {
        $descendantText = Get-DescendantText $window
        if ($descendantText -like "*Would you like to choose FeatherMark as the default app*") {
            Save-WindowScreenshot $window $screenshot
            $noButton = [FeatherMarkInstallerUi]::GetDlgItem($window, $noId)
            if ($noButton -eq [IntPtr]::Zero) {
                throw "The default-app prompt did not expose a No button."
            }
            [void][FeatherMarkInstallerUi]::SendMessage($noButton, $message, [IntPtr]::Zero, [IntPtr]::Zero)
            $captured = $true
            Write-Output "CapturedPrompt: $screenshot"
            break
        }
    }

    if (-not $captured) {
        foreach ($window in $windows) {
            $nextButton = [FeatherMarkInstallerUi]::GetDlgItem($window, $okId)
            if ($nextButton -ne [IntPtr]::Zero -and [FeatherMarkInstallerUi]::IsWindowEnabled($nextButton)) {
                [void][FeatherMarkInstallerUi]::SendMessage($nextButton, $message, [IntPtr]::Zero, [IntPtr]::Zero)
                break
            }
        }
    }
    else {
        foreach ($window in $windows) {
            $finishButton = [FeatherMarkInstallerUi]::GetDlgItem($window, $okId)
            if ($finishButton -ne [IntPtr]::Zero -and [FeatherMarkInstallerUi]::IsWindowEnabled($finishButton)) {
                [void][FeatherMarkInstallerUi]::SendMessage($finishButton, $message, [IntPtr]::Zero, [IntPtr]::Zero)
            }
        }
    }

    Start-Sleep -Milliseconds 300
    $process.Refresh()
}

if (-not $captured) {
    if (-not $process.HasExited) {
        $process.Kill()
    }
    throw "The installer default-app prompt was not captured."
}

if (-not $process.HasExited) {
    $process.WaitForExit(10000) | Out-Null
}

$uninstaller = Join-Path $env:LOCALAPPDATA "FeatherMark\uninstall.exe"
if (Test-Path -LiteralPath $uninstaller) {
    $uninstallProcess = Start-Process -FilePath $uninstaller -ArgumentList "/S" -PassThru
    $uninstallProcess.WaitForExit(15000) | Out-Null
    Write-Output "UninstallerExit: $($uninstallProcess.ExitCode)"
}

Write-Output "PromptVerified: $captured"
