<#
.SYNOPSIS
    Interactive Windows GUI Setup Wizard for SonyBridge.
#>

[CmdletBinding()]
param (
    [switch]$Silent,
    [string]$TargetDir = "$env:LOCALAPPDATA\Programs\SonyBridge"
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
if (-not $ScriptDir) { $ScriptDir = (Get-Location).Path }

$InstallPs1 = Join-Path $ScriptDir "install.ps1"
if ($Silent) {
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $InstallPs1 -TargetDir $TargetDir -NoLaunch
    exit 0
}

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
[System.Windows.Forms.Application]::EnableVisualStyles()

$Form = New-Object System.Windows.Forms.Form
$Form.Text = "SonyBridge v0.2.0 Setup Wizard"
$Form.Size = New-Object System.Drawing.Size(540, 390)
$Form.StartPosition = "CenterScreen"
$Form.FormBorderStyle = "FixedDialog"
$Form.MaximizeBox = $false
$Form.BackColor = [System.Drawing.Color]::FromArgb(24, 26, 32)
$Form.ForeColor = [System.Drawing.Color]::FromArgb(240, 242, 245)

$TitleLabel = New-Object System.Windows.Forms.Label
$TitleLabel.Text = "Install SonyBridge — WH-1000XM3 / XM4 / XM5 Companion"
$TitleLabel.Font = New-Object System.Drawing.Font("Segoe UI", 12, [System.Drawing.FontStyle]::Bold)
$TitleLabel.ForeColor = [System.Drawing.Color]::FromArgb(197, 160, 89)
$TitleLabel.Location = New-Object System.Drawing.Point(24, 20)
$TitleLabel.Size = New-Object System.Drawing.Size(480, 28)
$Form.Controls.Add($TitleLabel)

$DescLabel = New-Object System.Windows.Forms.Label
$DescLabel.Text = "Installs SonyBridge into your user profile (no Administrator privileges required), creates shortcuts, and configures Windows Bluetooth audio protection."
$DescLabel.Font = New-Object System.Drawing.Font("Segoe UI", 9)
$DescLabel.Location = New-Object System.Drawing.Point(24, 54)
$DescLabel.Size = New-Object System.Drawing.Size(480, 40)
$Form.Controls.Add($DescLabel)

$DirLabel = New-Object System.Windows.Forms.Label
$DirLabel.Text = "Installation Folder:"
$DirLabel.Location = New-Object System.Drawing.Point(24, 105)
$DirLabel.Size = New-Object System.Drawing.Size(200, 20)
$Form.Controls.Add($DirLabel)

$DirBox = New-Object System.Windows.Forms.TextBox
$DirBox.Text = $TargetDir
$DirBox.Location = New-Object System.Drawing.Point(24, 128)
$DirBox.Size = New-Object System.Drawing.Size(480, 24)
$Form.Controls.Add($DirBox)

$ChkDesktop = New-Object System.Windows.Forms.CheckBox
$ChkDesktop.Text = "Create Desktop shortcut"
$ChkDesktop.Checked = $true
$ChkDesktop.Location = New-Object System.Drawing.Point(24, 170)
$ChkDesktop.Size = New-Object System.Drawing.Size(450, 24)
$Form.Controls.Add($ChkDesktop)

$ChkStartMenu = New-Object System.Windows.Forms.CheckBox
$ChkStartMenu.Text = "Create Start Menu shortcut"
$ChkStartMenu.Checked = $true
$ChkStartMenu.Location = New-Object System.Drawing.Point(24, 200)
$ChkStartMenu.Size = New-Object System.Drawing.Size(450, 24)
$Form.Controls.Add($ChkStartMenu)

$ChkAutostart = New-Object System.Windows.Forms.CheckBox
$ChkAutostart.Text = "Launch SonyBridge automatically on Windows login"
$ChkAutostart.Checked = $true
$ChkAutostart.Location = New-Object System.Drawing.Point(24, 230)
$ChkAutostart.Size = New-Object System.Drawing.Size(450, 24)
$Form.Controls.Add($ChkAutostart)

$ChkLaunch = New-Object System.Windows.Forms.CheckBox
$ChkLaunch.Text = "Launch SonyBridge immediately after installation"
$ChkLaunch.Checked = $true
$ChkLaunch.Location = New-Object System.Drawing.Point(24, 260)
$ChkLaunch.Size = New-Object System.Drawing.Size(450, 24)
$Form.Controls.Add($ChkLaunch)

$BtnInstall = New-Object System.Windows.Forms.Button
$BtnInstall.Text = "Install Now"
$BtnInstall.BackColor = [System.Drawing.Color]::FromArgb(197, 160, 89)
$BtnInstall.ForeColor = [System.Drawing.Color]::Black
$BtnInstall.FlatStyle = "Flat"
$BtnInstall.Location = New-Object System.Drawing.Point(384, 305)
$BtnInstall.Size = New-Object System.Drawing.Size(120, 32)

$BtnInstall.Add_Click({
    $BtnInstall.Enabled = $false
    $BtnInstall.Text = "Installing..."
    $ArgsList = @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "`"$InstallPs1`"", "-TargetDir", "`"$($DirBox.Text)`"")
    if (-not $ChkDesktop.Checked) { $ArgsList += "-NoDesktopShortcut" }
    if (-not $ChkStartMenu.Checked) { $ArgsList += "-NoStartMenuShortcut" }
    if (-not $ChkAutostart.Checked) { $ArgsList += "-NoAutostart" }
    if (-not $ChkLaunch.Checked) { $ArgsList += "-NoLaunch" }

    $proc = Start-Process -FilePath "powershell.exe" -ArgumentList $ArgsList -Wait -PassThru -NoNewWindow
    if ($proc.ExitCode -eq 0) {
        [System.Windows.Forms.MessageBox]::Show("SonyBridge v0.2.0 has been installed!", "SonyBridge Setup", "OK", "Information") | Out-Null
        $Form.Close()
    } else {
        [System.Windows.Forms.MessageBox]::Show("Installation encountered an error (exit code $($proc.ExitCode)).", "SonyBridge Setup", "OK", "Error") | Out-Null
        $BtnInstall.Enabled = $true
        $BtnInstall.Text = "Install Now"
    }
})
$Form.Controls.Add($BtnInstall)

[void]$Form.ShowDialog()
