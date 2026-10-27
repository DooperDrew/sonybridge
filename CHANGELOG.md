# Changelog

All notable changes to **SonyBridge** are documented in this file.

## [Unreleased]

### Hardened & Fixed (Autonomous UI/UX & Accessibility QA via `ui-qa-tester`)
- **Accessibility & Contrast Conformance**:
  - Added explicit `aria-label` and `title` attributes to all sliders, preset selectors, equalizer bands, and toggles (`#17`).
  - Elevated contrast ratios across inactive tabs, secondary buttons, chips, and segmented controls for WCAG AA compliance (`#19`).
- **Layout & Interaction Polish**:
  - Resolved layout clipping on Architecture tab cards and footer by introducing custom scrollable viewports and adjusting window dimensions (`#16`).
  - Fixed First-Run Wizard button responsiveness, modal dismiss handling (close button, backdrop click, Escape key), and live MDR sequence string emission (`#18`).
- **Autonomous QA Harness**:
  - Integrated `qa-session.toml` and verified 0 defects across 15 crawl steps and 75 UIA elements using `ui-qa-tester`.

## [0.2.0] - 2026-10-25

### Added
- **Real Sony WH-1000XM3 MDR v1 Wire Protocol (`crates/sonybridge-core`)**:
  - Added authentic Sony MDR v1 byte-stuffed framing (`0x3E` start marker, `0x3C` end marker, `0x3D` escape byte with `0x10` mask, `0x0C` `DATA_MDR`, `0x01` `ACK`, and full header+payload modulo-256 checksum).
  - Added physical WH-1000XM3 MDR v1 command encoders (`0x68` `NCASM_SET_PARAM` for Dual NC, Wind Noise Reduction, and 20-step Ambient Sound + Voice Passthrough; `0x58` `EQEBB_SET_PARAM` for Clear Bass `-10..+10` and 5-band Equalizer; `0x12`/`0x13` Battery inquiry/reply; `0x28` Auto-Power Off; `0xD8` Touch Panel control).
- **Live WinRT RFCOMM Socket & Core Audio `IPolicyConfig` Routing (`crates/sonybridge-windows`)**:
  - Connected `WinRtRfcommTransport` over `Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService` and `StreamSocket` (`DataWriter`/`DataReader`) targeting Sony MDR v1 (`96cc203e-5068-46ad-b32d-396ceebe3928`), MDR v2 (`956c7b26-d49a-4ba8-b03f-b17d393cb6e2`), and SPP (`00001101-0000-1000-8000-00805f9b34fb`).
  - Implemented `AudioEndpointSanitizer::enforce_stereo_a2dp()` using the Windows Core Audio `IPolicyConfig` COM vtable to switch default playback across `eConsole`, `eMultimedia`, and `eCommunications` away from muffled `Hands-Free AG Audio` to high-res Stereo (`A2DP`).
  - Added `AutostartManager` for per-user `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\SonyBridge` registration.
- **3-Tab Adaptive UI, 1-Click Sound Profiles & First-Run Onboarding (`crates/sonybridge-app`)**:
  - Redesigned the tray flyout into 3 tabs (`Dashboard & EQ`, `Windows & Power`, `Architecture & Wire Inspector`).
  - Added 4 built-in 1-click Sound Profiles (*Focus (Max ANC)*, *Office / Calls*, *Cinema / Bass Boost*, *Studio Reference*).
  - Added interactive First-Run Onboarding Wizard modal and bilingual English/Italian (`EN`/`IT`) localization switcher.
  - Added live hexadecimal wire packet telemetry inspector.
- **Zero-Build Portable Release Packaging & Setup Wizard (`scripts/`, `packaging/`)**:
  - Added `scripts/package-release.ps1`, `scripts/setup.cmd`, `scripts/setup-wizard.ps1`, `scripts/install.ps1`, `scripts/uninstall.ps1`, and `scripts/generate-manifests.ps1` (Scoop & WinGet manifests).

## [0.1.0] - 2026-10-25

### Added
- Initial Rust Cargo workspace (`sonybridge-core`, `sonybridge-windows`, `sonybridge-cli`, `sonybridge-app`).
- Canonical MDR packet framing, state machine, mock transport, Core Audio endpoint enumeration, and Tauri v2 system tray application.
- Full specification suite (`docs/01-vision.md` through `docs/06-qa-and-validation.md`, `AGENTS.md`).
