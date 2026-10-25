<#
.SYNOPSIS
    Uninstalls SonyBridge for the current user.
#>

[CmdletBinding()]
param (
    [string]$TargetDir = "$env:LOCALAPPDATA\Programs\SonyBridge",
    [switch]$PurgeData,
    [switch]$Silent
)

$ErrorActionPreference = "Stop"

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "         SonyBridge User-Space Uninstaller        " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

# 1. Stop running processes
Get-Process -Name "sonybridge-app", "sonybridge-cli" -ErrorAction SilentlyContinue | ForEach-Object {
    try { Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue } catch {}
}
Start-Sleep -Milliseconds 300

# 2. Remove autostart registry entry
$RunKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
try {
    Remove-ItemProperty -Path $RunKey -Name "SonyBridge" -ErrorAction SilentlyContinue
} catch {}

# 3. Remove Uninstall registry entry
$UninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\SonyBridge"
if (Test-Path $UninstallKey) {
    Remove-Item -Path $UninstallKey -Recurse -Force -ErrorAction SilentlyContinue
}

# 4. Remove Shortcuts
$StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\SonyBridge"
if (Test-Path $StartMenuDir) {
    Remove-Item -Path $StartMenuDir -Recurse -Force -ErrorAction SilentlyContinue
}

$DesktopDir = [Environment]::GetFolderPath([Environment+SpecialFolder]::Desktop)
$DeskLnk = Join-Path $DesktopDir "SonyBridge.lnk"
if (Test-Path $DeskLnk) {
    Remove-Item -Path $DeskLnk -Force -ErrorAction SilentlyContinue
}

# 5. Remove installation directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
if (Test-Path $TargetDir) {
    if ($ScriptDir -and ($ScriptDir.TrimEnd('\') -eq $TargetDir.TrimEnd('\'))) {
        Get-ChildItem -Path $TargetDir -Exclude "uninstall.ps1" | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
        Start-Process cmd.exe -ArgumentList "/c cd /d %TEMP% & timeout /t 1 /nobreak >nul & rmdir /s /q `"$TargetDir`"" -WindowStyle Hidden
    } else {
        Remove-Item -Path $TargetDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

if ($PurgeData) {
    $AppDataDir = Join-Path $env:APPDATA "SonyBridge"
    if (Test-Path $AppDataDir) {
        Remove-Item -Path $AppDataDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Write-Host "`nSonyBridge has been uninstalled." -ForegroundColor Green
