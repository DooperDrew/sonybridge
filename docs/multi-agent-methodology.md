# Multi-Agent Engineering Methodology

SonyBridge is developed using a **spec-first, multi-agent orchestration methodology** modeled on `project-kickstart` and `multi-agent-delivery`.

## 1. Core Principles

1. **Knowledge Lives in the Repository**: Every architectural decision, protocol discovery, user correction, and milestone completion is persisted in `docs/`, `CHANGELOG.md`, `AGENTS.md`, and `docs/lessons-learned.md`.
2. **GitHub Governance (Milestones, Issues & Releases)**:
   - Every roadmap milestone (`M0` through `M10`) has a 1:1 **GitHub Milestone**.
   - Every deliverable task has a tracked **GitHub Issue** linked to its milestone.
   - Every release (`v0.1.0`, `v0.2.0`, …) is published on **GitHub Releases** with a pre-built portable `.zip` archive (`SonyBridge-v<version>-windows-x64.zip`) containing `sonybridge-app.exe`, `sonybridge-cli.exe`, and interactive setup wizards (`setup.cmd` / `setup-wizard.ps1`).
3. **Claims Follow Evidence**:
   - Headless protocol framing, byte-stuffing (`0x3D` escape), checksum calculation, state transitions, and JSON persistence are verified via deterministic unit tests (`cargo test --workspace`).
   - Physical hardware interaction with a live Sony WH-1000XM3 headset remains explicitly tracked in Milestone `M9` (`Pending` until empirical user verification).
4. **Pure Core Isolation**:
   - `crates/sonybridge-core` enforces `#![forbid(unsafe_code)]` and has zero OS or UI dependencies.
   - `crates/sonybridge-windows` isolates WinRT RFCOMM sockets and Windows Core Audio COM (`IMMDeviceEnumerator`, `IPolicyConfig`) behind clean safe Rust structs.

## 2. Complexity Tiers & Verification Gate

| Tier | Scope | Verification Gate |
|---|---|---|
| **T1 Lean** | MDR v1/v2 framing, byte-stuffing, checksums, EQ/ANC opcodes, i18n, config | `cargo test -p sonybridge-core` |
| **T2 Moderate** | Tauri v2 tray lifecycle, 3-tab glassmorphic UI, sound profiles, setup scripts | `cargo clippy --workspace --all-targets -- -D warnings` |
| **T3 Critical** | WinRT RFCOMM transport, `IPolicyConfig` A2DP routing, registry autostart | Full workspace build + release packaging |
