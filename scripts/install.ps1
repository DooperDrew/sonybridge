<#
.SYNOPSIS
    Installs SonyBridge for the current user without requiring Administrator privileges.

.DESCRIPTION
    Installs SonyBridge into %LOCALAPPDATA%\Programs\SonyBridge, creates Start Menu
    and Desktop shortcuts, registers the Windows Uninstall entry, configures autostart
    on Windows logon (HKCU), and launches the application.
#>

[CmdletBinding()]
param (
    [string]$TargetDir = "$env:LOCALAPPDATA\Programs\SonyBridge",
    [switch]$CreateDesktopShortcut,
    [switch]$NoDesktopShortcut,
    [switch]$CreateStartMenuShortcut,
    [switch]$NoStartMenuShortcut,
    [switch]$EnableAutostart,
    [switch]$NoAutostart,
    [switch]$LaunchNow,
    [switch]$NoLaunch
)

$ErrorActionPreference = "Stop"

$ResolvedDesktopShortcut = if ($NoDesktopShortcut) { $false } else { $true }
$ResolvedStartMenuShortcut = if ($NoStartMenuShortcut) { $false } else { $true }
$ResolvedAutostart = if ($NoAutostart) { $false } else { $true }
$ResolvedLaunchNow = if ($NoLaunch) { $false } else { $true }

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "          SonyBridge User-Space Installer         " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
if (-not $ScriptDir) { $ScriptDir = (Get-Location).Path }

$CandidateAppPaths = @(
    Join-Path $ScriptDir "sonybridge-app.exe",
    Join-Path $ScriptDir "..\target\release\sonybridge-app.exe",
    Join-Path $ScriptDir "target\release\sonybridge-app.exe"
)
$SourceApp = $CandidateAppPaths | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $SourceApp) {
    throw "sonybridge-app.exe not found in $ScriptDir or target\release."
}
$SourceDir = Split-Path -Parent $SourceApp
$Version = "0.2.0"

Write-Host "Source directory: $SourceDir"
Write-Host "Target directory: $TargetDir"

# Stop running instances if any
Get-Process -Name "sonybridge-app", "sonybridge-cli" -ErrorAction SilentlyContinue | ForEach-Object {
    try { Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue } catch {}
}
Start-Sleep -Milliseconds 300

if (-not (Test-Path $TargetDir)) {
    New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
}

Copy-Item (Join-Path $SourceDir "sonybridge-app.exe") -Destination $TargetDir -Force
$CliExe = Join-Path $SourceDir "sonybridge-cli.exe"
if (Test-Path $CliExe) {
    Copy-Item $CliExe -Destination $TargetDir -Force
}

$UninstallScript = Join-Path $ScriptDir "uninstall.ps1"
if (Test-Path $UninstallScript) {
    Copy-Item $UninstallScript -Destination $TargetDir -Force
}

# Copy locales
$TargetLocales = Join-Path $TargetDir "locales"
if (-not (Test-Path $TargetLocales)) {
    New-Item -ItemType Directory -Path $TargetLocales -Force | Out-Null
}
$LocalesSource = Join-Path $SourceDir "locales"
if (-not (Test-Path $LocalesSource)) {
    $LocalesSource = Join-Path $ScriptDir "..\crates\sonybridge-app\locales"
}
if (Test-Path $LocalesSource) {
    Get-ChildItem -Path $LocalesSource -Filter "*.toml" | ForEach-Object {
        Copy-Item $_.FullName -Destination $TargetLocales -Force
    }
}

$InstalledAppExe = Join-Path $TargetDir "sonybridge-app.exe"

# Shortcuts
$WshShell = New-Object -ComObject WScript.Shell
if ($ResolvedStartMenuShortcut) {
    $StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\SonyBridge"
    if (-not (Test-Path $StartMenuDir)) {
        New-Item -ItemType Directory -Path $StartMenuDir -Force | Out-Null
    }
    $Shortcut = $WshShell.CreateShortcut((Join-Path $StartMenuDir "SonyBridge.lnk"))
    $Shortcut.TargetPath = $InstalledAppExe
    $Shortcut.WorkingDirectory = $TargetDir
    $Shortcut.Description = "SonyBridge — WH-1000XM3 / XM4 / XM5 Desktop Companion"
    $Shortcut.Save()
}

if ($ResolvedDesktopShortcut) {
    $DesktopDir = [Environment]::GetFolderPath([Environment+SpecialFolder]::Desktop)
    if (Test-Path $DesktopDir) {
        $Shortcut = $WshShell.CreateShortcut((Join-Path $DesktopDir "SonyBridge.lnk"))
        $Shortcut.TargetPath = $InstalledAppExe
        $Shortcut.WorkingDirectory = $TargetDir
        $Shortcut.Description = "SonyBridge — WH-1000XM3 / XM4 / XM5 Desktop Companion"
        $Shortcut.Save()
    }
}

# Autostart registry key
$RunKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
if ($ResolvedAutostart) {
    Set-ItemProperty -Path $RunKey -Name "SonyBridge" -Value "`"$InstalledAppExe`"" -Type String
}

# Windows Add/Remove Programs Uninstall entry
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\SonyBridge"
if (-not (Test-Path $UninstallKey)) {
    New-Item -Path $UninstallKey -Force | Out-Null
}
$InstalledUninstallPs1 = Join-Path $TargetDir "uninstall.ps1"
Set-ItemProperty -Path $UninstallKey -Name "DisplayName" -Value "SonyBridge"
Set-ItemProperty -Path $UninstallKey -Name "DisplayVersion" -Value $Version
Set-ItemProperty -Path $UninstallKey -Name "Publisher" -Value "SonyBridge Contributors"
Set-ItemProperty -Path $UninstallKey -Name "InstallLocation" -Value $TargetDir
Set-ItemProperty -Path $UninstallKey -Name "DisplayIcon" -Value $InstalledAppExe
Set-ItemProperty -Path $UninstallKey -Name "UninstallString" -Value "powershell.exe -NoProfile -ExecutionPolicy Bypass -File `"$InstalledUninstallPs1`""
Set-ItemProperty -Path $UninstallKey -Name "NoModify" -Value 1 -Type DWord
Set-ItemProperty -Path $UninstallKey -Name "NoRepair" -Value 1 -Type DWord

if ($ResolvedLaunchNow) {
    Start-Process -FilePath $InstalledAppExe -WorkingDirectory $TargetDir
}

Write-Host "`nSonyBridge v$Version installed successfully to $TargetDir" -ForegroundColor Green
