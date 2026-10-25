<#
.SYNOPSIS
    Packages SonyBridge into a portable release ZIP archive for Windows x64.

.DESCRIPTION
    Builds the SonyBridge Cargo workspace in release mode, validates that all
    required binary and resource artifacts exist, stages them into a clean bundle
    (including setup-wizard.ps1, setup.cmd, install.ps1, uninstall.ps1, and locales),
    creates a portable zip archive in the output directory (default: dist/), and
    auto-generates WinGet and Scoop manifests.

.PARAMETER Version
    The release version string (e.g. "0.2.0"). If omitted, the version is
    extracted from the root Cargo.toml workspace.package.version field.

.PARAMETER OutputDir
    Destination folder for the final release ZIP (default: "dist").

.PARAMETER SkipBuild
    If set, skips running 'cargo build --workspace --release' and uses existing
    binaries in target/release.
#>

[CmdletBinding()]
param (
    [string]$Version,
    [string]$OutputDir = "dist",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..")
Set-Location $RepoRoot

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "        SonyBridge Release Packaging Tool         " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "Repository root: $RepoRoot"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if (Test-Path (Join-Path $CargoBin "cargo.exe")) {
        $env:PATH = "$CargoBin;$env:PATH"
    } else {
        throw "cargo was not found on PATH and was not located at $CargoBin\cargo.exe"
    }
}

if (-not $Version) {
    $CargoTomlPath = Join-Path $RepoRoot "Cargo.toml"
    if (Test-Path $CargoTomlPath) {
        $CargoTomlContent = Get-Content $CargoTomlPath -Raw
        if ($CargoTomlContent -match '\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"') {
            $Version = $matches[1]
            Write-Host "Extracted version from Cargo.toml: v$Version" -ForegroundColor Green
        }
    }
    if (-not $Version) {
        $Version = "0.2.0"
    }
}

# 1. Build workspace in release mode
if (-not $SkipBuild) {
    Write-Host "`n[1/5] Compiling workspace in release mode..." -ForegroundColor Cyan
    & cargo build --workspace --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo release build failed with exit code $LASTEXITCODE"
    }
    Write-Host "Release compilation succeeded." -ForegroundColor Green
} else {
    Write-Host "`n[1/5] Skipping build as requested (-SkipBuild)..." -ForegroundColor Yellow
}

# 2. Verify release binaries exist
Write-Host "`n[2/5] Verifying release binaries and assets..." -ForegroundColor Cyan
$AppExe = Join-Path $RepoRoot "target\release\sonybridge-app.exe"
$CliExe = Join-Path $RepoRoot "target\release\sonybridge-cli.exe"
$LocalesSrc = Join-Path $RepoRoot "crates\sonybridge-app\locales"
$ReadmeSrc = Join-Path $RepoRoot "README.md"
$LicenseSrc = Join-Path $RepoRoot "LICENSE"
$UserGuideSrc = Join-Path $RepoRoot "docs\user-guide.md"

if (-not (Test-Path $AppExe)) { throw "Required binary missing: $AppExe" }
if (-not (Test-Path $CliExe)) { throw "Required binary missing: $CliExe" }
if (-not (Test-Path $ReadmeSrc)) { throw "README.md missing: $ReadmeSrc" }
if (-not (Test-Path $LicenseSrc)) { throw "LICENSE missing: $LicenseSrc" }

Write-Host "  - Found desktop app binary: $AppExe" -ForegroundColor Green
Write-Host "  - Found diagnostic CLI binary: $CliExe" -ForegroundColor Green

# 3. Create clean staging directory
Write-Host "`n[3/5] Assembling staging directory..." -ForegroundColor Cyan
$AbsoluteOutputDir = Join-Path $RepoRoot $OutputDir
if (-not (Test-Path $AbsoluteOutputDir)) {
    New-Item -ItemType Directory -Path $AbsoluteOutputDir -Force | Out-Null
}

$ArchiveBaseName = "SonyBridge-v$Version-windows-x64"
$StagingDir = Join-Path $AbsoluteOutputDir "staging\$ArchiveBaseName"

if (Test-Path $StagingDir) {
    Remove-Item -Path $StagingDir -Recurse -Force
}
New-Item -ItemType Directory -Path $StagingDir -Force | Out-Null

# Copy binaries
Copy-Item -Path $AppExe -Destination (Join-Path $StagingDir "sonybridge-app.exe") -Force
Copy-Item -Path $CliExe -Destination (Join-Path $StagingDir "sonybridge-cli.exe") -Force

# Copy locales
if (Test-Path $LocalesSrc) {
    $StagingLocales = Join-Path $StagingDir "locales"
    New-Item -ItemType Directory -Path $StagingLocales -Force | Out-Null
    Copy-Item -Path (Join-Path $LocalesSrc "*.toml") -Destination $StagingLocales -Force
}

# Copy documentation & license
Copy-Item -Path $ReadmeSrc -Destination $StagingDir -Force
Copy-Item -Path $LicenseSrc -Destination $StagingDir -Force
if (Test-Path $UserGuideSrc) {
    Copy-Item -Path $UserGuideSrc -Destination $StagingDir -Force
}

# Copy installer and setup scripts
@("setup-wizard.ps1", "setup.cmd", "install.ps1", "uninstall.ps1") | ForEach-Object {
    $scriptPath = Join-Path $RepoRoot "scripts\$_"
    if (Test-Path $scriptPath) {
        Copy-Item -Path $scriptPath -Destination $StagingDir -Force
    }
}

Write-Host "Staging folder populated at: $StagingDir" -ForegroundColor Green

# 4. Create ZIP package
Write-Host "`n[4/5] Creating ZIP package..." -ForegroundColor Cyan
$ZipFilePath = Join-Path $AbsoluteOutputDir "$ArchiveBaseName.zip"
if (Test-Path $ZipFilePath) {
    Remove-Item -Path $ZipFilePath -Force
}

Compress-Archive -Path "$StagingDir\*" -DestinationPath $ZipFilePath -CompressionLevel Optimal
Remove-Item -Path (Join-Path $AbsoluteOutputDir "staging") -Recurse -Force

$ZipItem = Get-Item $ZipFilePath
$ZipSizeMB = [math]::Round($ZipItem.Length / 1MB, 2)

# 5. Generate WinGet & Scoop manifests
$GenerateManifestsScript = Join-Path $ScriptDir "generate-manifests.ps1"
if (Test-Path $GenerateManifestsScript) {
    Write-Host "`n[5/5] Generating distribution manifests (WinGet & Scoop)..." -ForegroundColor Cyan
    & powershell -NoProfile -ExecutionPolicy Bypass -File $GenerateManifestsScript -Version $Version -ArchivePath $ZipFilePath
}

Write-Host "`n==================================================" -ForegroundColor Green
Write-Host "       RELEASE PACKAGING COMPLETE SUCCESS         " -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host "Archive:  $ZipFilePath" -ForegroundColor Yellow
Write-Host "Size:     $ZipSizeMB MB ($($ZipItem.Length) bytes)" -ForegroundColor Yellow
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::OpenRead($ZipFilePath).Entries | ForEach-Object {
    Write-Host "  - $($_.FullName) ($([math]::Round($_.Length / 1KB, 1)) KB)" -ForegroundColor Gray
}
