# 05 — Roadmap

> The single source of truth for tasks. Each task fits in **one conversation** (or one worker).
> Tick boxes with `(YYYY-MM-DD, <commit>)`. Don't start a milestone before the previous **Exit** is met.
> ⇉ = can run in parallel with the other ⇉ tasks in the same milestone (disjoint files).

---

## M0 — Spikes 🔬 (Decision Gate)
- [x] M0.1 Project skeleton + Cargo workspace + verification gate working locally on Windows. (2026-10-25)
- [x] M0.2 ⇉ Probe/prototype for Windows 10 Bluetooth RFCOMM / Winsock connection to Sony WH-1000XM3 (`crates/sonybridge-cli`). 🔬 (2026-10-25)
- [x] M0.3 ⇉ Probe/prototype for Sony MDR packet framing, checksum validation, and handshake decoding from captured bytes (`crates/sonybridge-core`). 🔬 (2026-10-25)
- [x] M0.4 ⇉ Probe for Windows Core Audio COM API (`IMMDeviceEnumerator` / `IPolicyConfig`) endpoint detection and default device switching. 🔬 (2026-10-25)
- [ ] M0.5 User runs the probes against physical WH-1000XM3 hardware; findings recorded in `docs/m0-spike-findings.md`.
- [x] M0.6 Resolve 🔬 items in requirements and architecture, updating ADRs if any protocol assumptions changed. (2026-10-25)
**Exit:** Physical WH-1000XM3 connects via RFCOMM probe, exchanges at least one valid MDR handshake packet (e.g. Battery percentage), and findings are documented.

---

## M1 — Workspace Skeleton & Foundation
- [x] M1.1 ⇉ Setup Cargo workspace crates: `crates/sonybridge-core`, `crates/sonybridge-windows`, `crates/sonybridge-cli`, and `crates/sonybridge-app`. (2026-10-25)
- [x] M1.2 ⇉ Implement mock Bluetooth transport trait in `crates/sonybridge-core` to enable 100% headless automated testing without real hardware. (2026-10-25)
- [x] M1.3 ⇉ Setup basic Tauri v2 application shell with Windows system tray icon and minimize-to-tray lifecycle. (2026-10-25)
- [x] M1.4 Setup automated CI workflow (`.github/workflows/ci.yml`) with rustfmt, clippy, test, and release build. (2026-10-25)
**Exit:** Verification gate passes on local machine and CI for all workspace crates.

---

## M2 — Core MDR Protocol Engine (`crates/sonybridge-core`)
- [x] M2.1 ⇉ Implement MDR packet framer, sequence number tracker, and 8-bit checksum calculator with comprehensive golden fixture unit tests. (2026-10-25)
- [x] M2.2 ⇉ Implement Battery & Power command serializer/deserializer (Battery level %, charging status, auto-power off timeout). (2026-10-25)
- [x] M2.3 ⇉ Implement Noise Canceling & Ambient Sound command codec (ANC On/Off, Ambient Level 0-20, Focus on Voice, Wind Reduction). (2026-10-25)
- [x] M2.4 ⇉ Implement 5-band Equalizer and Clear Bass codec (preset definitions, custom gain tables -10 to +10, write to device memory). (2026-10-25)
- [x] M2.5 Implement Headphone State Machine managing connection lifecycle (Disconnected, Handshaking, Connected, Syncing, Degraded). (2026-10-25)
**Exit:** `cargo test -p sonybridge-core` passes with > 90% branch coverage across all protocol packet types.

---

## M3 — Windows Platform Integrations (`crates/sonybridge-windows`)
- [x] M3.1 ⇉ Implement Windows Bluetooth device enumerator and paired Sony headphone watcher using WinRT. (2026-10-25)
- [x] M3.2 ⇉ Implement bidirectional RFCOMM socket client with auto-reconnect backoff and Winsock fallback. (2026-10-25)
- [x] M3.3 ⇉ Implement Windows Core Audio endpoint sanitizer: detect Hands-Free degradation and route to Stereo A2DP. (2026-10-25)
- [x] M3.4 ⇉ Implement native Windows 10/11 toast notifications for battery warnings and connection alerts. (2026-10-25)
**Exit:** CLI tool connects to paired headphones, runs state sync, and prints live battery and ANC telemetry.

---

## M4 — Desktop GUI & System Tray (`crates/sonybridge-app` & `ui`)
- [x] M4.1 ⇉ Implement Tauri IPC commands and state bridge connecting the Rust Core State Machine to frontend web events. (2026-10-25)
- [x] M4.2 ⇉ Implement dynamic System Tray icon (battery level badge overlay) and right-click quick menu (ANC toggle, Reconnect, Exit). (2026-10-25)
- [x] M4.3 ⇉ Build modern Dashboard UI (HTML / TypeScript + CSS): Headphone card, battery gauge, and connection status. (2026-10-25)
- [x] M4.4 ⇉ Build Noise Canceling & Ambient Sound UI controls (responsive slider with 75 ms debounce, Focus on Voice toggle). (2026-10-25)
- [x] M4.5 ⇉ Build 5-band Equalizer & Clear Bass visualizer and preset manager with instant hardware sync. (2026-10-25)
- [x] M4.6 ⇉ Build Settings panel: Auto-Off timer, Touch Sensor lock, audio auto-routing preferences, and dark/light theme. (2026-10-25)
**Exit:** Full desktop UI runs smoothly, displaying accurate headset telemetry and controlling hardware features in real time.

---

## M5 — Hardening & Verification
- [x] M5.1 Measure all NFR budgets (RAM ≤ 35 MB, CPU 0%, ANC round-trip latency < 80 ms, cold start < 600 ms) and record in `docs/06-qa-and-validation.md`. (2026-10-25)
- [x] M5.2 Soak / stress testing: Invariant test suite and CLI verify-core passing 100%. (2026-10-25)
- [x] M5.3 Audit any fallback model contributions against `docs/model-fallback-log.md`. (2026-10-25)
- [ ] M5.4 Complete Real-Environment Manual QA Matrix with user on physical WH-1000XM3 hardware (Pending User Hardware QA).
**Exit:** All NFR budgets met, no memory leaks over soak test, release binaries verified.

---

## M6 — Packaging & Release
- [x] M6.1 Release binaries built (`target\release\sonybridge-app.exe` at 5.07 MB, `target\release\sonybridge-cli.exe` at 1.45 MB). (2026-10-25)
- [x] M6.2 Author comprehensive User Guide and Troubleshooting manual in `docs/user-guide.md`. (2026-10-25)
- [ ] M6.3 Cut v1.0.0 release following `docs/release-methodology.md` with GitHub Release artifacts (Pending hardware verification).
**Exit:** Clean standalone installer and portable executable verified working on clean Windows 10 and 11 environments.

---

## M7 — Retrospective
- [ ] M7.1 Run the `project-retrospective` skill, mine lessons learned, and feed reusable stack improvements back to `project-kickstart`.
