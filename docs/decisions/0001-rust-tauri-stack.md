# ADR 0001 — Technology Stack: Rust Core with Tauri v2 on Windows

- **Status:** Accepted
- **Date:** 2026-10-25
- **Deciders:** User & Antigravity (Lead Agent)

## Context
The goal is to build a high-performance, responsive Windows 10/11 desktop companion app and system tray utility for Sony WH-1000XM3 Bluetooth headphones (`sonybridge`).
Key constraints:
- Must run continuously in the background / system tray with minimal memory (budget ≤ 35 MB) and 0% idle CPU.
- Requires low-level Windows Bluetooth (RFCOMM/SPP, Winsock) and Windows Core Audio COM API access.
- Requires a polished, modern, responsive UI matching Windows dark/light themes.
- Core protocol logic must be pure, testable, and isolated from OS quirks.

## Options
| Option | Pros | Cons |
|---|---|---|
| **A: Rust + Tauri v2 (Accepted)** | • Ultra-low idle memory (~15-25 MB)<br>• Native Windows system tray & notification integration<br>• Direct access to Win32 / WinRT APIs via `windows-rs`<br>• Modern web frontend (Svelte/TypeScript) for fast, polished UI<br>• High-throughput, memory-safe Rust core | • Requires WebView2 runtime (pre-installed on Windows 10/11)<br>• Multi-language project (Rust + TS) |
| **B: Electron + Node.js** | • Rapid frontend development<br>• Large ecosystem | • Massive memory consumption (150-250 MB idle)<br>• Large installer (> 85 MB)<br>• C++ native addons (`node-gyp`) required for WinRT Bluetooth |
| **C: Pure C# / .NET MAUI / WPF** | • Native Windows SDK and WinRT integration | • High memory footprint (.NET CLR runtime ~60-90 MB)<br>• Not easily portable to cross-platform in future |
| **D: Pure Rust GUI (Slint / Iced / egui)** | • Single language, 100% native | • More complex styling for modern animations and sliders<br>• Ecosystem for complex web-like audio visualizers is less mature |

## Decision
**Option A: Rust workspace with Tauri v2.**
We will structure the project as a Cargo workspace:
- `crates/sonybridge-core`: Pure Rust crate containing protocol framing, state machine, and command codecs with `#![forbid(unsafe_code)]`.
- `crates/sonybridge-windows`: Windows Bluetooth RFCOMM/Winsock transport and Core Audio COM integration.
- `crates/sonybridge-cli`: Headless CLI diagnostic tool for M0 spikes and testing.
- `crates/sonybridge-app` (with Tauri v2): System tray daemon and modern WebView2 dashboard.

## Consequences
- **Positive:** Meets strict memory and CPU budgets. Core protocol is 100% unit-testable on any platform without hardware.
- **Positive:** Fast startup and native Windows feel.
- **Negative/Follow-up:** Need to ensure WebView2 is present (Tauri handles boot checks) and configure dual linting/formatting in CI for both Rust (`cargo fmt`/`clippy`) and the UI frontend.
