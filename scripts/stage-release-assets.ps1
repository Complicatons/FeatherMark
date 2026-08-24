param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('windows', 'macos', 'linux')]
    [string]$Platform,

    [Parameter(Mandatory = $true)]
    [ValidateSet('x64', 'aarch64')]
    [string]$Architecture
)

$ErrorActionPreference = 'Stop'

$projectRoot = Split-Path -Parent $PSScriptRoot
$configuration = Get-Content -LiteralPath (Join-Path $projectRoot 'src-tauri/tauri.conf.json') -Raw | ConvertFrom-Json
$version = $configuration.version
$destination = Join-Path $projectRoot 'dist/release'
New-Item -ItemType Directory -Path $destination -Force | Out-Null

function Copy-SingleBundle {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Pattern,

        [Parameter(Mandatory = $true)]
        [string]$DestinationName
    )

    $matches = @(Get-ChildItem -LiteralPath (Join-Path $projectRoot 'src-tauri/target') -Recurse -File -Filter $Pattern |
        Where-Object { $_.FullName -match '[\\/]release[\\/]bundle[\\/]' })
    if ($matches.Count -ne 1) {
        throw "Expected exactly one $Pattern release bundle, found $($matches.Count)."
    }
    Copy-Item -LiteralPath $matches[0].FullName -Destination (Join-Path $destination $DestinationName) -Force
}

switch ($Platform) {
    'windows' {
        $sourceDirectory = Join-Path $projectRoot 'dist/windows'
        $portable = Join-Path $sourceDirectory "FeatherMark-$version-windows-x64-portable.exe"
        $installer = Join-Path $sourceDirectory "FeatherMark-$version-windows-x64-setup.exe"
        foreach ($source in @($portable, $installer)) {
            if (-not (Test-Path -LiteralPath $source)) {
                throw "Missing Windows release asset: $source"
            }
            Copy-Item -LiteralPath $source -Destination $destination -Force
        }
    }
    'macos' {
        Copy-SingleBundle -Pattern '*.dmg' -DestinationName "FeatherMark-$version-macos-$Architecture.dmg"
    }
    'linux' {
        Copy-SingleBundle -Pattern '*.AppImage' -DestinationName "FeatherMark-$version-linux-$Architecture.AppImage"
        Copy-SingleBundle -Pattern '*.deb' -DestinationName "FeatherMark-$version-linux-$Architecture.deb"
    }
}

Get-ChildItem -LiteralPath $destination -File | Sort-Object Name | ForEach-Object {
    Write-Output $_.FullName
}
