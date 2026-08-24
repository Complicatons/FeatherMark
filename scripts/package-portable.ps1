$ErrorActionPreference = 'Stop'

$projectRoot = Split-Path -Parent $PSScriptRoot
$configurationPath = Join-Path $projectRoot 'src-tauri\tauri.conf.json'
$version = (Get-Content -LiteralPath $configurationPath -Raw | ConvertFrom-Json).version
$source = Join-Path $projectRoot 'src-tauri\target\release\feathermark.exe'
$destinationDirectory = Join-Path $projectRoot 'dist\windows'
$destination = Join-Path $destinationDirectory "FeatherMark-$version-windows-x64-portable.exe"

if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
    throw 'Build the release first with npm.cmd run build.'
}

New-Item -ItemType Directory -Path $destinationDirectory -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $destination -Force

$artifact = Get-Item -LiteralPath $destination
$stream = [System.IO.File]::OpenRead($destination)
try {
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    $hash = [System.BitConverter]::ToString($sha256.ComputeHash($stream)).Replace('-', '')
} finally {
    if ($sha256) { $sha256.Dispose() }
    $stream.Dispose()
}
Write-Output "Portable artifact: $($artifact.FullName)"
Write-Output "Size: $($artifact.Length) bytes"
Write-Output "SHA-256: $hash"
