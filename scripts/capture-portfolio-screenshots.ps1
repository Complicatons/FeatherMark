$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;

public static class FeatherMarkPortfolioCapture {
    [StructLayout(LayoutKind.Sequential)]
    public struct Rect {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Point {
        public int X;
        public int Y;
    }

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr window, out Rect rect);

    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr window);

    [DllImport("user32.dll")]
    public static extern bool SetWindowPos(IntPtr window, IntPtr insertAfter, int x, int y, int width, int height, uint flags);

    [DllImport("user32.dll")]
    public static extern bool SetCursorPos(int x, int y);

    [DllImport("user32.dll")]
    public static extern IntPtr WindowFromPoint(Point point);

    [DllImport("user32.dll")]
    public static extern bool ScreenToClient(IntPtr window, ref Point point);

    [DllImport("user32.dll")]
    public static extern bool PostMessage(IntPtr window, uint message, IntPtr word, IntPtr parameter);
}
"@

$projectRoot = Split-Path -Parent $PSScriptRoot
$configuration = Get-Content -LiteralPath (Join-Path $projectRoot 'src-tauri\tauri.conf.json') -Raw | ConvertFrom-Json
$executable = Join-Path $projectRoot "dist\windows\FeatherMark-$($configuration.version)-windows-x64-portable.exe"
$fixture = Join-Path $projectRoot 'fixtures\sample.md'
$qaDirectory = Join-Path $projectRoot 'qa'
New-Item -ItemType Directory -Path $qaDirectory -Force | Out-Null

function Save-ScreenRegion([IntPtr]$Window, [string]$Path) {
    $rect = [FeatherMarkPortfolioCapture+Rect]::new()
    if (-not [FeatherMarkPortfolioCapture]::GetWindowRect($Window, [ref]$rect)) {
        throw 'Could not read FeatherMark window bounds.'
    }
    $width = $rect.Right - $rect.Left
    $height = $rect.Bottom - $rect.Top
    $bitmap = [Drawing.Bitmap]::new($width, $height)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, $bitmap.Size)
        $bitmap.Save($Path, [Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
    throw 'Build and package the portable release first.'
}

$process = Start-Process -FilePath $executable -ArgumentList "`"$fixture`"" -PassThru
$topmost = [IntPtr](-1)
$notTopmost = [IntPtr](-2)
$showWindow = 0x0040
$window = [IntPtr]::Zero

try {
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    while ($window -eq [IntPtr]::Zero -and [DateTime]::UtcNow -lt $deadline) {
        Start-Sleep -Milliseconds 250
        $process.Refresh()
        $window = [IntPtr]$process.MainWindowHandle
    }
    if ($window -eq [IntPtr]::Zero) {
        throw 'FeatherMark did not expose a main window in time.'
    }

    [void][FeatherMarkPortfolioCapture]::SetWindowPos($window, $topmost, 80, 80, 1056, 789, $showWindow)
    [void][FeatherMarkPortfolioCapture]::SetForegroundWindow($window)
    Start-Sleep -Seconds 2

    $readerPath = Join-Path $qaDirectory 'portfolio-reader.png'
    Save-ScreenRegion $window $readerPath

    $rect = [FeatherMarkPortfolioCapture+Rect]::new()
    if (-not [FeatherMarkPortfolioCapture]::GetWindowRect($window, [ref]$rect)) {
        throw 'Could not position the context-menu click.'
    }
    $screenPoint = [FeatherMarkPortfolioCapture+Point]::new()
    $screenPoint.X = $rect.Left + 430
    $screenPoint.Y = $rect.Top + 225
    [void][FeatherMarkPortfolioCapture]::SetCursorPos($screenPoint.X, $screenPoint.Y)
    $targetWindow = [FeatherMarkPortfolioCapture]::WindowFromPoint($screenPoint)
    $clientPoint = $screenPoint
    [void][FeatherMarkPortfolioCapture]::ScreenToClient($targetWindow, [ref]$clientPoint)
    $packedPoint = [IntPtr](($clientPoint.Y -shl 16) -bor ($clientPoint.X -band 0xffff))
    [void][FeatherMarkPortfolioCapture]::PostMessage($targetWindow, 0x0204, [IntPtr]2, $packedPoint)
    [void][FeatherMarkPortfolioCapture]::PostMessage($targetWindow, 0x0205, [IntPtr]0, $packedPoint)
    Start-Sleep -Milliseconds 700

    $contextPath = Join-Path $qaDirectory 'portfolio-context-menu.png'
    Save-ScreenRegion $window $contextPath

    Write-Output "Reader: $readerPath"
    Write-Output "ContextMenu: $contextPath"
}
finally {
    if ($window -ne [IntPtr]::Zero) {
        [void][FeatherMarkPortfolioCapture]::SetWindowPos($window, $notTopmost, 0, 0, 0, 0, 0x0003)
    }
    if (-not $process.HasExited) {
        [void]$process.CloseMainWindow()
        if (-not $process.WaitForExit(5000)) {
            $process.Kill()
        }
    }
}
