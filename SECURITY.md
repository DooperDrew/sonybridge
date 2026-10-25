# Security Policy for SonyBridge

The **SonyBridge** team takes software integrity, user-mode audio safety, and privacy seriously.

---

## 1. Supported Versions

Security updates and critical patches are released for the following versions:

| Version | Supported | Notes |
|---|---|---|
| `v0.2.x` | ✅ Yes | Current stable release branch |
| `v0.1.x` | ❌ No | Please upgrade to `v0.2.0` |

---

## 2. Core Security & Safety Guarantees

SonyBridge is engineered under strict non-functional security and safety constraints:

1. **Zero Kernel Drivers & Zero Elevation:** Strictly **no kernel-mode virtual audio drivers (`.sys`)**, no driver signing bypasses, and no UAC Administrator elevation required. SonyBridge runs 100% in standard user space (`%LOCALAPPDATA%\Programs\SonyBridge`).
2. **Pure Safe Rust Protocol Core (`#![forbid(unsafe_code)]`):** All Sony MDR v1/v2 packet parsing, byte-stuffing (`0x3D`), and checksum validation live in `crates/sonybridge-core` with `#![forbid(unsafe_code)]`, eliminating buffer overflows or memory corruption from malformed Bluetooth packets.
3. **Scoped COM & Audio Endpoint Integrity:** All Windows Core Audio (`IMMDeviceEnumerator`, `IPolicyConfig`) COM pointers in `crates/sonybridge-windows` are strictly scoped before `CoUninitialize()` and never mute or detach endpoints without graceful fallback.
4. **100% Offline & Privacy-Preserving:** SonyBridge makes **zero network connections**, runs no telemetry or analytics, and requires no Sony cloud account or login.

---

## 3. Reporting a Vulnerability

If you discover a potential security vulnerability or safety issue in SonyBridge:
- Please report it via [GitHub Private Vulnerability Reporting](https://github.com/DooperDrew/sonybridge/security/advisories/new) or open an issue if non-sensitive.
- We will acknowledge receipt within 48 hours and coordinate a timely fix and release.
