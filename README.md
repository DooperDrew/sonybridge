# 🎧 SonyBridge

[![CI](https://github.com/DooperDrew/sonybridge/actions/workflows/ci.yml/badge.svg)](https://github.com/DooperDrew/sonybridge/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/DooperDrew/sonybridge?label=Download%20ZIP&color=38bdf8)](https://github.com/DooperDrew/sonybridge/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Lightweight, native Windows 10/11 system tray companion for Sony Bluetooth headphones (`WH-1000XM3`, `WH-1000XM4`, `WH-1000XM5`, and `WF` series).**
> Solves Windows Bluetooth Hands-Free AG audio degradation and brings the full functionality of the mobile *Sony | Headphones Connect* app to the PC desktop: **real-time battery monitoring**, **Active Noise Canceling (ANC) & 20-step Ambient Sound**, **5-band hardware Equalizer & Clear Bass**, **1-click Sound Profiles**, and **smart Windows Stereo (A2DP) audio routing**.

---

| 🚀 [**Download Portable `.zip` (v0.2.0)**](https://github.com/DooperDrew/sonybridge/releases/download/v0.2.0/SonyBridge-v0.2.0-windows-x64.zip) | 📖 [**User Guide**](docs/user-guide.md) | 🗺️ [**Roadmap & Milestones**](https://github.com/DooperDrew/sonybridge/milestones) | 🐛 [**Issues & Hardware QA**](https://github.com/DooperDrew/sonybridge/issues) |
| :--- | :--- | :--- | :--- |

---

## ✨ Key Features

- **Real Sony MDR Protocol v1 & v2 over RFCOMM**: Communicates directly with your Sony WH-1000XM3 (`96cc203e-5068-46ad-b32d-396ceebe3928`) using authentic byte-stuffed (`0x3E`..`0x3C`, `0x3D` escape) `DATA_MDR` frames and verified checksums.
- **Active Noise Canceling (ANC) & Ambient Sound**: Switch instantaneously between **ANC On (Dual Noise Sensor)**, **Wind Noise Reduction**, **20-level Ambient Sound** (`1..20`) with **Focus on Voice** passthrough, and **Off**.
- **Hardware 5-Band Equalizer & Clear Bass**: Adjust hardware DSP frequencies (`400 Hz`, `1 kHz`, `2.5 kHz`, `6.3 kHz`, `16 kHz`) from `-10` to `+10` dB and **Clear Bass** (`-10..+10`) directly on the headset.
- **1-Click Sound Profiles**: Switch between *Focus (Max ANC)*, *Office / Calls*, *Cinema / Bass Boost*, and *Studio Reference* with a single click.
- **Smart Windows Audio Endpoint Sanitizer (`IPolicyConfig`)**: Detects and prevents Windows from routing music or system audio into the muffled 8 kHz mono `Hands-Free AG Audio` (HFP) profile, automatically enforcing high-res Stereo (`A2DP`) across Console, Multimedia, and Communications roles.
- **Live Wire Frame Inspector**: Built-in hexadecimal protocol inspector lets you inspect every outgoing and incoming MDR packet in real time.
- **Bilingual UI (`EN` / `IT`) & First-Run Onboarding Wizard**: Instant English/Italian localization and guided setup modal.

---

## 📦 Installation (No Build Required!)

### Option 1 — Portable `.zip` with Interactive Setup Wizard (Recommended)
1. Download **[`SonyBridge-v0.2.0-windows-x64.zip`](https://github.com/DooperDrew/sonybridge/releases/download/v0.2.0/SonyBridge-v0.2.0-windows-x64.zip)** from [GitHub Releases](https://github.com/DooperDrew/sonybridge/releases/latest).
2. Extract the archive to any folder.
3. Double-click **`setup.cmd`** to open the interactive Setup Wizard:
   - **`[1]` Quick Launch (Portable)** — Starts `sonybridge-app.exe` immediately.
   - **`[2]` Install to `%LOCALAPPDATA%\Programs\SonyBridge`** — Creates a Start Menu shortcut and optional Windows Startup registration (no administrator privileges required).
   - **`[3]` Run Headless Diagnostic Probe** — Runs `sonybridge-cli.exe probe` to inspect paired Sony headphones, RFCOMM services, and Windows Core Audio endpoints.
   - **`[4]` Uninstall** — Cleanly removes shortcuts and startup entries.

### Option 2 — Direct PowerShell Installer
From an extracted release folder:
```powershell
.\install.ps1 -RegisterStartup
```

---

## 🏗️ Workspace Architecture

| Crate | Path | Responsibility |
|---|---|---|
| [`sonybridge-core`](crates/sonybridge-core) | `crates/sonybridge-core` | `#![forbid(unsafe_code)]` pure Rust Sony MDR v1/v2 wire protocol (`0x3D` byte stuffing, checksums, ANC `0x68`, EQ `0x58`, Battery `0x12`), state machine, sound profiles, and EN/IT i18n. |
| [`sonybridge-windows`](crates/sonybridge-windows) | `crates/sonybridge-windows` | Windows WinRT RFCOMM `StreamSocket` transport, Core Audio `IMMDeviceEnumerator` + `IPolicyConfig` A2DP routing sanitizer, and `HKCU\...\Run` startup manager. |
| [`sonybridge-cli`](crates/sonybridge-cli) | `crates/sonybridge-cli` | Headless diagnostic CLI (`sonybridge-cli probe` and `sonybridge-cli enforce-a2dp`). |
| [`sonybridge-app`](crates/sonybridge-app) | `crates/sonybridge-app` | Tauri v2 Windows 10/11 system tray companion with 3-tab glassmorphic flyout UI, onboarding wizard, and live wire inspector. |

---

## ⚡ Resource & Performance Budgets

| Metric | Target Budget | Design Guarantee |
|---|---|---|
| **Idle RAM Footprint** | `<= 35 MB` | Native Rust + shared Windows 10/11 Edge WebView2 runtime |
| **Idle CPU Usage** | `0.0%` | Event-driven RFCOMM I/O & UI state transitions (zero background polling loops) |
| **Binary Size** | `< 10 MB` | LTO-optimized release binary (`~3.8 MB` for `sonybridge-app.exe`) |

---

## 🛠️ Building from Source

```powershell
# Run full verification gate
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Build release executables
cargo build --workspace --release

# Package portable release archive + generate Scoop/WinGet manifests
.\scripts\package-release.ps1 -Version "0.2.0" -SkipBuild
.\scripts\generate-manifests.ps1 -Version "0.2.0"
```

---

## 📚 Documentation Index

- [Documentation Hierarchy & Reading Order (`docs/README.md`)](docs/README.md)
- [01 — Vision & Problem Statement](docs/01-vision.md)
- [02 — Requirements & Budgets](docs/02-requirements.md)
- [03 — System Architecture](docs/03-architecture.md)
- [04 — UX & System Tray Specification](docs/04-ux.md)
- [05 — Roadmap & Milestones](docs/05-roadmap.md)
- [06 — QA & Hardware Validation Matrix](docs/06-qa-and-validation.md)
- [07 — UX Enhancements & v0.2.0 Evolution](docs/07-ux-and-improvements.md)
- [08 — Packaging & Distribution Strategy](docs/08-community-and-growth-strategy.md)
- [Multi-Agent & GitHub Copilot Cloud Methodology](docs/multi-agent-methodology.md)
- [Release & Versioning Methodology](docs/release-methodology.md)
- [M0 Protocol & Core Audio Spike Findings](docs/m0-spike-findings.md)
- [User Guide](docs/user-guide.md)
- [Lessons Learned](docs/lessons-learned.md)
- [Model Fallback Log](docs/model-fallback-log.md)
- [Contributing & Agent Workflow](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
