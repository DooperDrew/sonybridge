# 08 — Packaging, Distribution & Community Strategy

## 1. Zero-Build Portable Release ZIP
Every tagged release on GitHub (`v0.1.0`, `v0.2.0`, …) ships with a self-contained portable archive:
- `SonyBridge-v<version>-windows-x64.zip`

Contents of the `.zip`:
- `sonybridge-app.exe` — Native Windows 10/11 system tray application
- `sonybridge-cli.exe` — Headless Bluetooth RFCOMM & Core Audio diagnostic CLI
- `setup.cmd` & `setup-wizard.ps1` — Interactive double-click setup & launch wizard
- `install.ps1` & `uninstall.ps1` — Per-user Start Menu + Startup installer (no admin rights required)
- `locales/en.toml` & `locales/it.toml` — Bilingual English and Italian UI strings
- `README.md`, `LICENSE`, `CHANGELOG.md`, `docs/` — Full offline documentation

## 2. Package Manager Manifests
Generated automatically via `.\scripts\generate-manifests.ps1`:
- **Scoop**: `packaging/scoop/sonybridge.json`
- **WinGet**: `packaging/winget/DooperDrew.SonyBridge.yaml`
