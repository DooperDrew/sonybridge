#Requires -Version 5.1
<#
.SYNOPSIS
    Generates Scoop and WinGet package manifests for SonyBridge from a packaged release ZIP.
#>
param(
    [string]$Version = "0.2.0",
    [string]$GitHubOwner = "DooperDrew",
    [string]$GitHubRepo = "sonybridge"
)

$ErrorActionPreference = "Stop"
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$ZipPath = Join-Path $RepoRoot "dist\SonyBridge-v$Version-windows-x64.zip"

if (-not (Test-Path $ZipPath)) {
    throw "Release archive not found at $ZipPath. Run .\scripts\package-release.ps1 first."
}

$Hash = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash
$HashLower = $Hash.ToLowerInvariant()
$DownloadUrl = "https://github.com/$GitHubOwner/$GitHubRepo/releases/download/v$Version/SonyBridge-v$Version-windows-x64.zip"

$ScoopDir = Join-Path $RepoRoot "packaging\scoop"
$WingetDir = Join-Path $RepoRoot "packaging\winget"
New-Item -ItemType Directory -Path $ScoopDir -Force | Out-Null
New-Item -ItemType Directory -Path $WingetDir -Force | Out-Null

$ScoopJson = @"
{
    "version": "$Version",
    "description": "Lightweight Windows 10/11 system tray companion for Sony Bluetooth headphones (WH-1000XM3/XM4/XM5): real-time battery, ANC/Ambient Sound, 5-band Equalizer & Clear Bass, and smart A2DP audio routing.",
    "homepage": "https://github.com/$GitHubOwner/$GitHubRepo",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "$DownloadUrl",
            "hash": "$HashLower"
        }
    },
    "bin": [
        "sonybridge-app.exe",
        "sonybridge-cli.exe"
    ],
    "shortcuts": [
        [
            "sonybridge-app.exe",
            "SonyBridge"
        ]
    ],
    "checkver": "github",
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/$GitHubOwner/$GitHubRepo/releases/download/v`$version/SonyBridge-v`$version-windows-x64.zip"
            }
        }
    }
}
"@

$ScoopPath = Join-Path $ScoopDir "sonybridge.json"
Set-Content -Path $ScoopPath -Value $ScoopJson -Encoding utf8

$WingetYaml = @"
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.singleton.1.6.0.schema.json
PackageIdentifier: $GitHubOwner.SonyBridge
PackageVersion: $Version
PackageLocale: en-US
Publisher: $GitHubOwner
PackageName: SonyBridge
PackageUrl: https://github.com/$GitHubOwner/$GitHubRepo
License: MIT
ShortDescription: Windows 10/11 system tray companion for Sony WH-1000XM3/XM4/XM5 headphones (Battery, ANC, 5-Band EQ, Clear Bass, A2DP routing).
ManifestType: singleton
ManifestVersion: 1.6.0
Installers:
  - Architecture: x64
    InstallerType: zip
    NestedInstallerType: portable
    NestedInstallerFiles:
      - RelativeFilePath: sonybridge-app.exe
        PortableCommandAlias: sonybridge
      - RelativeFilePath: sonybridge-cli.exe
        PortableCommandAlias: sonybridge-cli
    InstallerUrl: $DownloadUrl
    InstallerSha256: $Hash
"@

$WingetPath = Join-Path $WingetDir "$GitHubOwner.SonyBridge.yaml"
Set-Content -Path $WingetPath -Value $WingetYaml -Encoding utf8

Write-Host "Generated manifests:" -ForegroundColor Green
Write-Host "  Scoop : $ScoopPath"
Write-Host "  WinGet: $WingetPath"
Write-Host "  SHA256: $Hash"
