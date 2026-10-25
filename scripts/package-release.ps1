<#
.SYNOPSIS
    Builds and packages SonyBridge into a distributable portable ZIP archive and generates WinGet/Scoop manifests.

.PARAMETER Version
    Override the version string. If omitted, extracted from workspace Cargo.toml.

.PARAMETER SkipBuild
    If specified, skips running 'cargo build --workspace --release' and uses existing binaries.

.EXAMPLE
    .\scripts\package-release.ps1
    .\scripts\package-release.ps1 -Version "0.2.0" -SkipBuild
#>

[CmdletBinding()]
param (
    [string]$Version,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$RepoRoot = (Resolve-Path (Join-Path $ScriptDir "..")).Path

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "        SonyBridge Release Packaging Tool         " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "Repository root: $RepoRoot"

if ([string]::IsNullOrWhiteSpace($Version)) {
    $CargoTomlPath = Join-Path $RepoRoot "Cargo.toml"
    $CargoContent = Get-Content $CargoTomlPath -Raw
    if ($CargoContent -match '\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"') {
        $Version = $matches[1]
    } else {
        $Version = "0.2.0"
    }
}

$PackageName = "SonyBridge-v$Version-windows-x64"
$DistDir = Join-Path $RepoRoot "dist"
$StageDir = Join-Path $DistDir "staging\$PackageName"
$ZipPath = Join-Path $DistDir "$PackageName.zip"

if (-not $SkipBuild) {
    Write-Host "`n[1/5] Building workspace in release mode..." -ForegroundColor Cyan
    Push-Location $RepoRoot
    try {
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
        cargo build --workspace --release
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build --workspace --release failed with exit code $LASTEXITCODE"
        }
    } finally {
        Pop-Location
    }
} else {
    Write-Host "`n[1/5] Skipping build as requested (-SkipBuild)..." -ForegroundColor Yellow
}

Write-Host "`n[2/5] Verifying release binaries and assets..." -ForegroundColor Cyan
$ReleaseDir = Join-Path $RepoRoot "target\release"
$AppExe = Join-Path $ReleaseDir "sonybridge-app.exe"
$CliExe = Join-Path $ReleaseDir "sonybridge-cli.exe"

if (-not (Test-Path $AppExe)) { throw "Missing release binary: $AppExe" }
if (-not (Test-Path $CliExe)) { throw "Missing release binary: $CliExe" }

Write-Host "`n[3/5] Assembling staging directory..." -ForegroundColor Cyan
if (Test-Path $StageDir) { Remove-Item -Path $StageDir -Recurse -Force }
New-Item -ItemType Directory -Path $StageDir -Force | Out-Null

Copy-Item $AppExe -Destination $StageDir -Force
Copy-Item $CliExe -Destination $StageDir -Force

$LocalesSrc = Join-Path $RepoRoot "crates\sonybridge-app\locales"
$LocalesDst = Join-Path $StageDir "locales"
New-Item -ItemType Directory -Path $LocalesDst -Force | Out-Null
Get-ChildItem -Path $LocalesSrc -Filter "*.toml" | ForEach-Object {
    Copy-Item $_.FullName -Destination $LocalesDst -Force
}

@("install.ps1", "uninstall.ps1", "setup-wizard.ps1", "setup.cmd", "installer.iss") | ForEach-Object {
    $ScriptFile = Join-Path $ScriptDir $_
    if (Test-Path $ScriptFile) {
        Copy-Item $ScriptFile -Destination $StageDir -Force
    }
}

Copy-Item (Join-Path $RepoRoot "README.md") -Destination $StageDir -Force
Copy-Item (Join-Path $RepoRoot "LICENSE") -Destination $StageDir -Force
Copy-Item (Join-Path $RepoRoot "CHANGELOG.md") -Destination $StageDir -Force
$UserGuidePath = Join-Path $RepoRoot "docs\user-guide.md"
if (Test-Path $UserGuidePath) {
    Copy-Item $UserGuidePath -Destination (Join-Path $StageDir "user-guide.md") -Force
}

Write-Host "`n[4/5] Creating ZIP package..." -ForegroundColor Cyan
if (Test-Path $ZipPath) { Remove-Item -Path $ZipPath -Force }
Compress-Archive -Path "$StageDir\*" -DestinationPath $ZipPath -CompressionLevel Optimal -Force

Write-Host "`n[5/5] Generating distribution manifests (WinGet & Scoop)..." -ForegroundColor Cyan
$ManifestGen = Join-Path $ScriptDir "generate-manifests.ps1"
if (Test-Path $ManifestGen) {
    & $ManifestGen -Version $Version -ArchivePath $ZipPath
}

$ZipInfo = Get-Item $ZipPath
$SizeMB = [math]::Round($ZipInfo.Length / 1MB, 2)
Write-Host "`n==================================================" -ForegroundColor Green
Write-Host "       RELEASE PACKAGING COMPLETE SUCCESS         " -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host "Archive:  $ZipPath ($SizeMB MB)"
