# SonyBridge 🎧

[![CI](https://github.com/DooperDrew/sonybridge/actions/workflows/ci.yml/badge.svg)](https://github.com/DooperDrew/sonybridge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.99%2B-orange.svg)](https://www.rust-lang.org)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app)

A lightweight, responsive Windows 10/11 desktop companion and system tray application for Sony Bluetooth headphones (specifically the **WH-1000XM3**, **WH-1000XM4**, **WH-1000XM5**, and **WF** series).

SonyBridge brings the full functionality of the mobile "Sony | Headphones Connect" app directly to the PC desktop with an ultra-lightweight native footprint (~22 MB idle RAM, 0% CPU).

---

## ✨ Features

- **🔋 Real-time Battery Monitoring in Taskbar Tray:** Always know your exact headphone battery percentage and charging state at a glance.
- **🔇 Hardware Noise Canceling & Ambient Sound:** Toggle Noise Canceling, Off, or adjust Ambient Sound across 20 granular levels with optional "Focus on Voice".
- **🎛️ 5-Band Hardware Equalizer & Clear Bass:** Tune sound profiles (-10 dB to +10 dB across 400Hz, 1kHz, 2.5kHz, 6.3kHz, 16kHz) and Clear Bass. Configurations persist directly into the headphone's onboard EEPROM memory.
- **⚡ Anti-Hands-Free Audio Routing:** One-click enforcement of high-bitrate Stereo A2DP to prevent Windows from routing audio to degraded telephony profiles.
- **🔒 Earcup Touch Sensor Lock & Auto-Power Off:** Toggle swipe gesture controls and set automatic idle shutdown timers.
- **🪟 Windows Taskbar System Tray:** Left-click toggles the dashboard; closing the window hides to the tray so background monitoring stays active.

---

## 🚀 Quick Start

### Pre-compiled Binaries (in `target/release/`)
- **Desktop Companion App:** `target/release/sonybridge-app.exe` (5.07 MB)
- **Diagnostic Probe Utility:** `target/release/sonybridge-cli.exe` (1.45 MB)

### Running the Diagnostic Probe (CLI):
Ensure your Sony headphones are paired in Windows Settings and run:
```powershell
.\target\release\sonybridge-cli.exe probe
```

### Launching the Desktop App:
```powershell
.\target\release\sonybridge-app.exe
```

---

## 🏗️ Architecture

Structured as a Cargo workspace with pure logic isolation:
- `crates/sonybridge-core`: Pure Sony MDR (Music Device Remote) protocol framing, 8-bit checksum calculation, command codecs, and connection state machine (`#![forbid(unsafe_code)]`, zero OS dependencies).
- `crates/sonybridge-windows`: Windows Bluetooth discovery (WinRT) and Windows Core Audio COM endpoint enumeration (`IMMDeviceEnumerator`).
- `crates/sonybridge-cli`: Headless CLI diagnostic tool for probing and testing.
- `crates/sonybridge-app`: Tauri v2 desktop application with system tray and web UI.

---

## 🧪 Development & Verification Gate

```powershell
# Check formatting
cargo fmt --all -- --check

# Clippy lints (warnings as errors)
cargo clippy --workspace --all-targets -- -D warnings

# Unit & invariant test suite
cargo test --workspace

# Production release build
cargo build --workspace --release
```

---

## 📖 Documentation

- [User Guide & Hardware Testing Manual](docs/user-guide.md)
- [01 — Vision](docs/01-vision.md)
- [02 — Requirements](docs/02-requirements.md)
- [03 — Architecture](docs/03-architecture.md)
- [04 — UX & Controls](docs/04-ux.md)
- [05 — Roadmap](docs/05-roadmap.md)
- [06 — QA & Validation Matrix](docs/06-qa-and-validation.md)
- [M0 Spike Findings](docs/m0-spike-findings.md)
- [ADR 0001: Rust Core + Tauri v2 Stack](docs/decisions/0001-rust-tauri-stack.md)

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
