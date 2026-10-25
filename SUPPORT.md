# Getting Support for SonyBridge 🎧 🌉

Welcome to the **SonyBridge** support guide! Whether you need help pairing your Sony WH-1000XM3 / XM4 / XM5 headphones on Windows, fixing muffled Bluetooth audio, or submitting a hardware QA report, here is where to find answers:

---

## 1. Quick Documentation Links

Before opening an issue, check if your question is covered in our documentation:

- 📖 **[User Guide & Manual](docs/user-guide.md)** — Complete installation methods (WinGet, Scoop, Guided Setup Wizard, Portable ZIP), ANC/Ambient controls, 5-Band EQ, A2DP audio routing, and uninstallation.
- 🪄 **[Guided Setup Wizard](scripts/setup-wizard.ps1)** — Zero-elevation Windows Forms graphical installer (`setup.cmd` / `setup-wizard.ps1`).
- 🧪 **[Hardware QA Verification Matrix](docs/06-qa-and-validation.md)** — Empirical hardware test protocols for Sony WH-1000XM3 / XM4 / XM5 headphones.
- 📦 **[Package Distribution & WinGet](docs/release-methodology.md)** — Installation via WinGet, Scoop, or portable ZIP.
- 🤝 **[Community & Ecosystem Roadmap](docs/08-community-and-growth-strategy.md)** — Multi-model Sony expansion plan (`WH-1000XM3`, `WH-1000XM4`, `WH-1000XM5`, `WF-1000XM5`, `LinkBuds`).

---

## 2. Common Questions & Troubleshooting

### Q: Why does my Sony headphone audio sound muffled or mono on Windows?
**A:** When an application accesses the microphone, Windows Bluetooth switches your headphones from high-resolution **Stereo (`A2DP`)** into the legacy 8 kHz mono **`Hands-Free AG Audio` (`HFP`)** telephony profile. Open SonyBridge &rarr; **Windows & Power** tab and click **Enforce High-Res Stereo (A2DP)** (or run `sonybridge-cli.exe enforce-a2dp`).

### Q: Do I need administrator rights to install SonyBridge?
**A:** **No.** SonyBridge runs entirely in user space. The guided setup wizard (`setup.cmd` / `setup-wizard.ps1`) installs into `%LOCALAPPDATA%\Programs\SonyBridge`, creates Start Menu and Desktop shortcuts, and registers in Windows **Installed apps** without requiring UAC elevation.

### Q: How do I inspect my paired headphone's Bluetooth RFCOMM services?
**A:** Run `.\sonybridge-cli.exe probe` in PowerShell, or open the **Architecture & Wire Inspector** tab inside `sonybridge-app.exe` and click **Run RFCOMM Diagnostic Probe**.

---

## 3. How to Report Issues & Submit QA Reports

- **Bug Reports:** Open a [Bug Report](https://github.com/DooperDrew/sonybridge/issues/new?template=bug_report.yml) with your Windows build version and headphone model.
- **Hardware QA Submissions:** Run `.\sonybridge-cli.exe probe` and paste the output into [Issue #10](https://github.com/DooperDrew/sonybridge/issues/10) or open a [Hardware QA Report](https://github.com/DooperDrew/sonybridge/issues/new?template=hardware_qa_report.md).
- **Feature Requests:** Submit a [Feature Request](https://github.com/DooperDrew/sonybridge/issues/new?template=feature_request.yml).
