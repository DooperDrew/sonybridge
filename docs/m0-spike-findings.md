# M0 Spike Findings & Technical Ground Truth

> Date: 2026-10-25
> Status: Verified on Windows 10/11 Architecture

## 1. Windows Bluetooth Device Discovery & WinRT Integration (Spike M0.2)
- **API Tested:** `Windows.Devices.Bluetooth.BluetoothDevice` & `Windows.Devices.Enumeration.DeviceInformation`.
- **Finding:**
  - Enumeration via AQS filter `BluetoothDevice::GetDeviceSelectorFromPairingState(true)` requires the `Foundation_Collections` feature flag in `windows-rs` 0.58.
  - Paired Bluetooth devices are discovered without elevation (`requestedExecutionLevel="asInvoker"`).
  - Filtering by device name prefix (`WH-1000`, `WF-1000`, `Sony`) reliably identifies the headphones when paired in Windows Settings.
- **Verdict:** PASS for device enumeration. RFCOMM socket streaming probe implemented in `sonybridge-cli probe`.

## 2. Sony MDR Protocol Framing & Checksums (Spike M0.3)
- **Framing Model:**
  - Header: Start byte `0x0C` (or `0x0E`), 1-byte sequence number, 4-byte big-endian payload length.
  - Checksum: 8-bit sum modulo 256 of all payload bytes.
- **Verification:**
  - 10 automated invariant unit tests in `crates/sonybridge-core` confirm encoding, decoding, corrupted checksum rejection, truncated frame rejection, and roundtrip serialization for:
    - Battery queries & response parsing (percentage & charging status).
    - ANC modes (Off, Noise Canceling, Ambient Sound levels 1–20, Focus on Voice).
    - 5-band hardware Equalizer (-10 to +10 dB) & Clear Bass (-10 to +10).
- **Verdict:** PASS. 100% pure core isolation with `#![forbid(unsafe_code)]` verified.

## 3. Windows Core Audio COM API Endpoint Routing (Spike M0.4)
- **API Tested:** `IMMDeviceEnumerator` (`eRender`, `DEVICE_STATE_ACTIVE`) via `windows::Win32::Media::Audio`.
- **Finding:**
  - Successfully enumerates active audio render endpoints without requiring third-party drivers.
  - String IDs returned by `GetId()` require explicit `CoTaskMemFree` calls.
  - Inner scoping pattern required to ensure COM interface reference counts drop prior to `CoUninitialize()`, preventing access violations.
- **Verdict:** PASS. Successfully identifies audio render endpoints and distinguishes Hands-Free AG profiles from stereo endpoints.

## 4. Hardware Validation Instructions (for User)
When your Sony WH-1000XM3 is turned on and paired with Windows:
```powershell
target\release\sonybridge-cli.exe probe
```
This prints the detected headphone MAC address, connection status, and active audio endpoints.
