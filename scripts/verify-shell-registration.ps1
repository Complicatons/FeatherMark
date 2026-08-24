$ErrorActionPreference = 'Stop'

$projectRoot = Split-Path -Parent $PSScriptRoot
$configuration = Get-Content -LiteralPath (Join-Path $projectRoot 'src-tauri\tauri.conf.json') -Raw | ConvertFrom-Json
$installer = Join-Path $projectRoot "src-tauri\target\release\bundle\nsis\FeatherMark_$($configuration.version)_x64-setup.exe"
$installDirectory = Join-Path $env:LOCALAPPDATA 'FeatherMark'
$executable = Join-Path $installDirectory 'feathermark.exe'
$uninstaller = Join-Path $installDirectory 'uninstall.exe'
$openCommand = "`"$executable`" `"%1`""

function Get-UserChoice([string]$Extension) {
    $path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\$Extension\UserChoice"
    return (Get-ItemProperty -LiteralPath $path -Name ProgId -ErrorAction SilentlyContinue).ProgId
}

function Assert-RegistryValue([string]$Path, [string]$Name, [AllowEmptyString()][string]$Expected) {
    if (-not (Test-Path -LiteralPath $Path)) {
        throw "Missing registry key: $Path"
    }

    $key = Get-Item -LiteralPath $Path
    if ($key.GetValueNames() -notcontains $Name) {
        throw "Missing registry value '$Name' under $Path"
    }

    $actual = [string]$key.GetValue($Name)
    if ($actual -ne $Expected) {
        throw "Unexpected value '$Name' under $Path. Expected '$Expected', found '$actual'."
    }
}

function Assert-RegistryKeyAbsent([string]$Path) {
    if (Test-Path -LiteralPath $Path) {
        throw "Registry key remained after uninstall: $Path"
    }
}

if (-not (Test-Path -LiteralPath $installer -PathType Leaf)) {
    throw 'Build the NSIS installer first with npm.cmd run build.'
}

if ((Test-Path -LiteralPath $installDirectory) -or
    (Test-Path -LiteralPath 'HKCU:\Software\FeatherMark\Capabilities')) {
    throw 'Refusing to run because FeatherMark is already installed for this user.'
}

$mdDefaultBefore = Get-UserChoice '.md'
$markdownDefaultBefore = Get-UserChoice '.markdown'
$installed = $false

try {
    $installProcess = Start-Process -FilePath $installer -ArgumentList '/S' -PassThru
    $installProcess.WaitForExit(60000) | Out-Null
    if (-not $installProcess.HasExited -or $installProcess.ExitCode -ne 0) {
        throw "Installer failed or timed out. Exit code: $($installProcess.ExitCode)"
    }
    $installed = $true

    if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
        throw "Installed executable was not found at $executable"
    }

    Assert-RegistryValue 'HKCU:\Software\RegisteredApplications' 'FeatherMark' 'Software\FeatherMark\Capabilities'
    Assert-RegistryValue 'HKCU:\Software\FeatherMark\Capabilities\FileAssociations' '.md' 'FeatherMark.Markdown'
    Assert-RegistryValue 'HKCU:\Software\FeatherMark\Capabilities\FileAssociations' '.markdown' 'FeatherMark.Markdown'

    Assert-RegistryValue 'HKCU:\Software\Classes\Applications\feathermark.exe' 'FriendlyAppName' 'FeatherMark'
    Assert-RegistryValue 'HKCU:\Software\Classes\Applications\feathermark.exe\SupportedTypes' '.md' ''
    Assert-RegistryValue 'HKCU:\Software\Classes\Applications\feathermark.exe\SupportedTypes' '.markdown' ''
    Assert-RegistryValue 'HKCU:\Software\Classes\Applications\feathermark.exe\shell\open\command' '' $openCommand

    Assert-RegistryValue 'HKCU:\Software\Classes\.md\OpenWithProgids' 'FeatherMark.Markdown' ''
    Assert-RegistryValue 'HKCU:\Software\Classes\.markdown\OpenWithProgids' 'FeatherMark.Markdown' ''

    Assert-RegistryValue 'HKCU:\Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open' 'MUIVerb' 'Open with FeatherMark'
    Assert-RegistryValue 'HKCU:\Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open\command' '' $openCommand
    Assert-RegistryValue 'HKCU:\Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open' 'MUIVerb' 'Open with FeatherMark'
    Assert-RegistryValue 'HKCU:\Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open\command' '' $openCommand

    if ((Get-UserChoice '.md') -ne $mdDefaultBefore -or
        (Get-UserChoice '.markdown') -ne $markdownDefaultBefore) {
        throw 'The silent installer unexpectedly changed a user default.'
    }

    Write-Output 'Installed Open with registration: verified'
    Write-Output 'Installed context-menu verbs: verified'
    Write-Output 'Existing user defaults unchanged: verified'
}
finally {
    if ($installed -and (Test-Path -LiteralPath $uninstaller -PathType Leaf)) {
        $uninstallProcess = Start-Process -FilePath $uninstaller -ArgumentList '/S' -PassThru
        $uninstallProcess.WaitForExit(60000) | Out-Null
        if (-not $uninstallProcess.HasExited -or $uninstallProcess.ExitCode -ne 0) {
            throw "Uninstaller failed or timed out. Exit code: $($uninstallProcess.ExitCode)"
        }
    }
}

$cleanupDeadline = [DateTime]::UtcNow.AddSeconds(10)
while ([DateTime]::UtcNow -lt $cleanupDeadline -and
    ((Test-Path -LiteralPath $installDirectory) -or
     (Test-Path -LiteralPath 'HKCU:\Software\FeatherMark\Capabilities') -or
     (Test-Path -LiteralPath 'HKCU:\Software\Classes\Applications\feathermark.exe'))) {
    Start-Sleep -Milliseconds 200
}

Assert-RegistryKeyAbsent 'HKCU:\Software\FeatherMark\Capabilities'
Assert-RegistryKeyAbsent 'HKCU:\Software\Classes\Applications\feathermark.exe'
Assert-RegistryKeyAbsent 'HKCU:\Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open'
Assert-RegistryKeyAbsent 'HKCU:\Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open'
Assert-RegistryKeyAbsent 'HKCU:\Software\Classes\FeatherMark.Markdown'

foreach ($extension in '.md', '.markdown') {
    $path = "HKCU:\Software\Classes\$extension\OpenWithProgids"
    if (Test-Path -LiteralPath $path) {
        $names = (Get-Item -LiteralPath $path).GetValueNames()
        if ($names -contains 'FeatherMark.Markdown') {
            throw "OpenWithProgids value remained after uninstall for $extension"
        }
    }
}

Write-Output 'Uninstall registration cleanup: verified'
