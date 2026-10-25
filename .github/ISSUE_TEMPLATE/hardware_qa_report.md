---
name: Hardware QA Verification Report
about: Submit an empirical verification report for Sony Bluetooth headphones (WH-1000XM3 / XM4 / XM5 / WF)
title: "[QA]: Hardware Verification on Sony <MODEL>"
labels: ["qa", "hardware"]
assignees: ""
---

### Hardware Information
- **Sony Headphone Model:** [e.g. WH-1000XM3, WH-1000XM4, WH-1000XM5, WF-1000XM5]
- **Headphone Firmware Version (if known):** [e.g. v4.5.2]
- **Bluetooth Adapter / Connection:** [e.g. Intel AX210 Bluetooth 5.3]
- **Windows OS Version:** [e.g. Windows 11 23H2 x64, Windows 10 22H2 x64]
- **SonyBridge Version:** [e.g. v0.2.0]

### Verification Checklist (Milestone M9)
- [ ] `sonybridge-cli.exe probe` discovers paired Sony headphone and lists RFCOMM service UUID (`96cc203e-5068-46ad-b32d-396ceebe3928` or `956c7b26-d49a-4ba8-b03f-b17d393cb6e2`)
- [ ] Live battery percentage (`0x12`/`0x13`) reported accurately in dashboard and tray tooltip
- [ ] ANC On (Dual Noise Sensor) / Wind Noise Reduction / Ambient Sound (`1..20`) + Focus on Voice (`0x68`) transitions audibly on headset
- [ ] 5-Band Equalizer (`400 Hz`..`16 kHz`) & Clear Bass (`-10..+10`, opcode `0x58`) updates headphone hardware DSP in real time
- [ ] 1-Click Sound Profiles (*Focus*, *Office / Calls*, *Cinema / Bass Boost*, *Studio Reference*) switch ANC + EQ cleanly
- [ ] 1-Click **Enforce High-Res Stereo (A2DP)** switches Windows default playback away from muffled `Hands-Free AG Audio`
- [ ] Windows Sleep/Resume and Bluetooth out-of-range recovery reconnect cleanly

### Attached `sonybridge-cli.exe probe` Output
<!-- Paste the JSON output of `.\sonybridge-cli.exe probe` and any wire hex frames from the Architecture & Wire Inspector tab below -->
```json
<paste probe output here>
```

### Notes & Observations
<!-- Any quirks, delays, or unexpected behavior observed -->
