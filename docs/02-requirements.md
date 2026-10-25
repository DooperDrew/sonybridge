# 02 — Requirements

> Status: **APPROVED**. Priority: **M**ust / **S**hould / **C**ould.
> 🔬 = depends on a Milestone 0 spike; it isn't verified yet.

## Functional requirements

### F1 — Device Discovery & Bluetooth Connection (M)
- F1.1 Detect paired Sony WH-1000XM3 (and compatible Sony MDR series) via Windows Bluetooth device enumeration.
- F1.2 Establish a reliable control channel over Bluetooth RFCOMM / SPP (Serial Port Profile) using the Sony MDR protocol GUID. 🔬 (Linked to Spike M0.2)
- F1.3 **Fallback:** If WinRT RFCOMM socket creation fails or encounters Windows 10 permission blocks, fallback to classic Win32 Winsock (`AF_BTH` / `WSASocketWithReceive`) socket. 🔬 (Linked to Spike M0.2)
- F1.4 Provide a 1-click "Connect / Reconnect" button in the tray and main window that wakes the connection without unpairing.
- F1.5 Automatic background reconnection when the headphones power on within Bluetooth range.

### F2 — Sony MDR Protocol Framing & Negotiation (M)
- F2.1 Encode and decode Sony MDR Protocol v1 / v2 packet frames (Start byte `0x0C` / `0x0E`, Sequence Number, Payload Length, Payload Data, Checksum byte). 🔬 (Linked to Spike M0.3)
- F2.2 Implement the initial connection handshake: Query Protocol Version, Device Info, Supported Capabilities (Capabilities Query). 🔬 (Linked to Spike M0.3)
- F2.3 Handle unsolicited async notification packets sent by the headphones when physical buttons (e.g., NC/Ambient button) are pressed.

### F3 — Battery Monitoring & Notifications (M)
- F3.1 Query and display current battery level (percentage 0–100%) and charging state in the UI.
- F3.2 Display real-time battery status icon/tooltip directly on the Windows Taskbar System Tray.
- F3.3 Send native Windows 10/11 toast notification when battery reaches configurable low thresholds (e.g., 20%, 10%).

### F4 — Active Noise Canceling (ANC) & Ambient Sound Control (M)
- F4.1 Query current ANC / Ambient Sound state from the headphones on connection.
- F4.2 Provide mode switching:
  - Noise Canceling: Full ON.
  - Wind Noise Reduction (if supported by firmware).
  - Ambient Sound: 20-step slider (Level 1 to 20).
  - Focus on Voice toggle (vocal band pass-through).
  - Ambient Sound / NC completely OFF.
- F4.3 Instantly transmit parameter updates to the headphones via MDR command opcodes without audio disruption.

### F5 — Hardware Equalizer & Clear Bass (M)
- F5.1 Read active Equalizer profile and Clear Bass level from the headphones.
- F5.2 Provide 5-band frequency adjustment (400 Hz, 1 kHz, 2.5 kHz, 6.3 kHz, 16 kHz: range -10 to +10 dB).
- F5.3 Clear Bass slider control (range -10 to +10 dB).
- F5.4 Built-in preset selection (Bright, Excited, Mellow, Relaxed, Vocal, Treble Boost, Bass Boost, Speech, Off, Custom 1, Custom 2).
- F5.5 Persist EQ configurations directly into the headphone's non-volatile hardware memory.

### F6 — Device Settings & Power Management (S)
- F6.1 Auto-Power Off idle timeout selector (Off / 5 min / 30 min / 1 hour / 3 hours).
- F6.2 Touch sensor panel toggle (enable/disable earcup touch controls to avoid accidental taps).
- F6.3 Voice guidance toggle and volume level adjustment.
- F6.4 Display firmware version, MAC address, and model identifier.

### F7 — Windows Audio Endpoint Sanitizer (S)
- F7.1 Detect when Windows assigns the headphones to the Hands-Free AG profile during normal audio playback. 🔬 (Linked to Spike M0.4)
- F7.2 Provide option to automatically set "Headphones (Stereo A2DP)" as the default multimedia playback device upon connection.
- F7.3 **Fallback:** If COM programmatic device switching requires elevated privileges or fails, notify user with a 1-click shortcut to Windows Sound Control Panel.

### F8 — System Tray & Desktop Experience (M)
- F8.1 Minimize-to-tray by default with custom tray icon reflecting connection and battery status.
- F8.2 Tray context menu with quick toggles: Quick Reconnect, ANC Mode (NC / Ambient / Off), Battery Status, and Exit.
- F8.3 Clean, modern responsive window (built with Tauri v2 and modern web UI) matching Windows 10/11 dark/light themes.

---

## Non-functional requirements

### Performance (hard budgets, verified in M5 hardening)
| ID | Metric | Budget | How it's measured |
|---|---|---|---|
| NFR-P1 | Idle RAM footprint (in tray) | ≤ 35 MB Private Working Set | Windows Task Manager / Process Explorer test |
| NFR-P2 | Idle CPU usage | 0.0% (event-driven, no busy loops) | Performance Monitor trace over 5 minutes |
| NFR-P3 | ANC command round-trip latency | < 80 ms from click to headphone ack | High-resolution timer in core benchmark |
| NFR-P4 | Cold startup time to tray | < 600 ms | Timestamped launch benchmark |
| NFR-P5 | Reconnect acquisition time | < 2.5 s after headphones turn on | Logged timestamp delta on RFCOMM open |

### Safety, security & compatibility
- **NFR-S1:** No administrative rights required (`requestedExecutionLevel="asInvoker"`).
- **NFR-S2:** Zero network connectivity or external telemetry; 100% offline and air-gapped safe.
- **NFR-S3:** Safe protocol bounds checking: malformed Bluetooth packets must be dropped gracefully without crashing the daemon or corrupting memory.
- **NFR-S4:** Compatible with Windows 10 (Build 19041+) and Windows 11 (21H2+).

### Quality & Maintainability
- **NFR-Q1:** `crates/sonybridge-core` has zero unsafe code (`#![forbid(unsafe_code)]`), zero platform dependencies, and > 90% unit test branch coverage.
- **NFR-Q2:** Automated verification gate enforces `cargo fmt`, `cargo clippy -- -D warnings`, and clean test runs.
- **NFR-Q3:** Diagnostics and Bluetooth packet logging disabled by default; toggled via an explicit debug flag or UI setting.
