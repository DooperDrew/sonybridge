# 07 — UX Enhancements & Product Evolution (v0.2.0)

Following the initial `v0.1.0` architectural scaffold, SonyBridge evolved in `v0.2.0` to deliver a complete, zero-friction desktop experience modeled after best-in-class Windows tray utilities.

## 1. Real-World Sony WH-1000XM3 MDR v1 Wire Protocol (`#12`)
- **Byte-Stuffed RFCOMM Framing**: Added full support for the physical Sony MDR v1 wire format (`0x3E` start marker, `0x3C` end marker, `0x3D` escape byte with `0x10` XOR/masking, `0x0C` `DATA_MDR` data type, `0x01` `ACK` frame, and modulo-256 checksum over `[data_type, seq, len[4], payload...]`).
- **Physical WH-1000XM3 Opcodes**:
  - `0x68` (`NCASM_SET_PARAM`): Dual Noise Sensor ANC, Wind Noise Reduction, and 20-step Ambient Sound (`1..=20`) with Voice Passthrough (`Focus on Voice`).
  - `0x58` (`EQEBB_SET_PARAM`): Hardware Clear Bass (`-10..=+10`) and 5-band Equalizer (`400Hz`, `1kHz`, `2.5kHz`, `6.3kHz`, `16kHz`).
  - `0x12` / `0x13` (`BATTERY_GET_LEVEL` / `BATTERY_RET_LEVEL`): Real-time battery telemetry.
  - `0x28` (`POWER_OFF_SET_PARAM`) and `0xD8` (`SYSTEM_SET_PARAM` touch panel lock).

## 2. 3-Tab Adaptive Flyout & 1-Click Sound Profiles (`#13`)
- **Tab 1 — Dashboard & EQ**: Quick 1-click Sound Profile bar (*Focus (Max ANC)*, *Office / Calls*, *Cinema / Bass Boost*, *Studio Reference*), ANC/Wind/Ambient segmented selector, Voice Passthrough toggle, Clear Bass slider, and 5-band EQ sliders.
- **Tab 2 — Windows & Power**: 1-click **Enforce High-Res Stereo (A2DP)** button using Windows Core Audio `IPolicyConfig::SetDefaultEndpoint`, Auto-Switch A2DP on Connect toggle, Start with Windows (`HKCU\...\Run`) toggle, Auto-Power Off selector, and Right-Ear Touch Sensor lock.
- **Tab 3 — Architecture & Wire Inspector**: Live hexadecimal wire frame inspector displaying the exact MDR v1 byte-stuffed packet transmitted on every slider or button interaction, plus 1-click RFCOMM diagnostic probe.

## 3. Interactive Setup Wizard & First-Run Onboarding (`#14`)
- **First-Run Onboarding Modal**: Guides new users through pairing their WH-1000XM3/XM4/XM5, enabling Windows A2DP stereo protection, and selecting an initial Sound Profile.
- **Double-Click `setup.cmd` Wizard**: Interactive console menu (`[1] Quick Launch`, `[2] Install to %LOCALAPPDATA%\Programs\SonyBridge`, `[3] Run Diagnostic Probe`, `[4] Uninstall`) so users never have to compile Rust or open a terminal manually.
