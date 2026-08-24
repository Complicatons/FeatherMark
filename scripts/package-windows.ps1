$ErrorActionPreference = 'Stop'

$projectRoot = Split-Path -Parent $PSScriptRoot
$configurationPath = Join-Path $projectRoot 'src-tauri\tauri.conf.json'
$version = (Get-Content -LiteralPath $configurationPath -Raw | ConvertFrom-Json).version
$portableScript = Join-Path $PSScriptRoot 'package-portable.ps1'
$installerSource = Join-Path $projectRoot "src-tauri\target\release\bundle\nsis\FeatherMark_${version}_x64-setup.exe"
$destinationDirectory = Join-Path $projectRoot 'dist\windows'
$installerDestination = Join-Path $destinationDirectory "FeatherMark-$version-windows-x64-setup.exe"

& $portableScript

if (-not (Test-Path -LiteralPath $installerSource -PathType Leaf)) {
    throw 'Build the Windows installer first with npm.cmd run build.'
}

Copy-Item -LiteralPath $installerSource -Destination $installerDestination -Force

$artifact = Get-Item -LiteralPath $installerDestination
$stream = [System.IO.File]::OpenRead($installerDestination)
try {
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    $hash = [System.BitConverter]::ToString($sha256.ComputeHash($stream)).Replace('-', '')
}
finally {
    if ($sha256) { $sha256.Dispose() }
    $stream.Dispose()
}

Write-Output "Installer artifact: $($artifact.FullName)"
Write-Output "Size: $($artifact.Length) bytes"
Write-Output "SHA-256: $hash"
