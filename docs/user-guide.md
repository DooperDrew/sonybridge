# SonyBridge — User Guide & Hardware Testing Manual

Welcome to **SonyBridge**, your native Windows 10/11 desktop companion for Sony Bluetooth headphones (specifically WH-1000XM3, plus XM4, XM5, and WF series).

---

## 1. Quick Start (Running the Application)

The release binaries are located in `target\release\`:
- **Desktop Companion App:** `target\release\sonybridge-app.exe` (5.07 MB)
- **Headless Diagnostic Probe:** `target\release\sonybridge-cli.exe` (1.45 MB)

### Launching the Desktop GUI:
Double-click `target\release\sonybridge-app.exe` or run in PowerShell:
```powershell
.\target\release\sonybridge-app.exe
```

---

## 2. Features Walkthrough

### 1. Windows System Tray
- **Tray Icon:** SonyBridge runs silently in the taskbar notification area.
- **Left-Click:** Instantly toggles the main dashboard window (show/hide).
- **Right-Click Context Menu:**
  - `Sony WH-1000XM3 (85%)` — Live battery and device status.
  - `Quick Reconnect` — Forces an instant Bluetooth reconnection.
  - `Open Dashboard` — Restores the dashboard to the foreground.
  - `Quit SonyBridge` — Exits the application cleanly.
- **Close Button (X):** Clicking the window close button minimizes SonyBridge to the system tray so battery monitoring and ANC stay active in the background.

### 2. Device Telemetry & Battery Gauge
- Real-time battery percentage readout with visual fill gauge.
- Charging state indicators (Charging / Discharging / Fully Charged).

### 3. Active Noise Canceling (ANC) & Ambient Sound Control
- **Modes:**
  - **Noise Canceling:** Full hardware ANC active.
  - **Ambient Sound:** 20-step hardware transparency slider.
  - **Off:** Complete bypass for natural passive isolation and maximum battery life.
- **Focus on Voice:** Emphasizes speech frequencies while suppressing low-frequency hum (air conditioning, engines).
- **Smooth Debouncing:** Sliders automatically debounce transmissions (75 ms) to prevent Bluetooth buffer congestion.

### 4. 5-Band Hardware Equalizer & Clear Bass
- **DSP Presets:** Flat / Off, Excited, Bright, Mellow, Relaxed, Vocal, Treble Boost, Bass Boost, Speech, Custom 1, Custom 2.
- **5 Adjustable Bands:** 400 Hz, 1 kHz, 2.5 kHz, 6.3 kHz, 16 kHz (range: -10 dB to +10 dB).
- **Clear Bass Slider:** Sony's proprietary sub-bass enhancement (-10 to +10).
- **Hardware Sync:** Equalizer curves are written directly into headphone EEPROM memory so presets persist when switching to other devices.

### 5. Windows Audio Endpoint Sanitizer
- **Enforce A2DP:** Detects if Windows has mistakenly assigned the headphones to the muffled "Hands-Free AG Telephony" audio device and re-routes audio to high-bitrate "Stereo A2DP".
- **Touch Sensor Toggle:** Enable/disable earcup touch swipe gestures to prevent accidental taps.
- **Auto-Power Off Timeout:** Configure idle shutdown (5 min, 30 min, 1 hr, 3 hr, or never).

---

## 3. Hardware Testing Procedure (For Tomorrow)

Follow these steps when testing with your physical WH-1000XM3 headphones:

### Step 1: Headless Diagnostic Check
With headphones powered on and paired with Windows:
```powershell
.\target\release\sonybridge-cli.exe probe
```
**Expected:** The CLI should output your headphone name (e.g., `Sony WH-1000XM3`), Bluetooth MAC address, and list your active audio render endpoints.

### Step 2: Launch GUI & Check Tray
```powershell
.\target\release\sonybridge-app.exe
```
**Expected:** The dark-and-gold themed dashboard appears, displaying your headphone battery level. A tray icon appears in the Windows notification area.

### Step 3: Test Hardware Controls
1. Toggle between **Noise Canceling** and **Off**. Listen for the audible change in headphone noise isolation.
2. Select **Ambient Sound** and move the slider from 1 to 20. Confirm surrounding environment transparency increases.
3. Switch Equalizer preset to **Excited** or **Bass Boost** while playing music in Windows. Confirm the tone changes immediately in your ears.
4. Close the window with the `[X]` button. Confirm the app minimizes to the system tray rather than terminating.

---

## 4. Troubleshooting
- **Device Not Detected:** Ensure the headphones are connected under Windows Settings -> Bluetooth & devices.
- **Audio Sounds Muffled:** Click **"Enforce A2DP"** in the SonyBridge dashboard or verify that "Headphones (WH-1000XM3 Stereo)" is selected in the Windows taskbar volume flyout.
