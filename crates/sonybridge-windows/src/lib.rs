//! Windows platform-specific integrations for SonyBridge.
//!
//! Covers:
//! - Bluetooth RFCOMM / WinRT StreamSocket transport
//! - Audio endpoint detection and default routing (A2DP vs Hands-Free via `IPolicyConfig`)
//! - Per-user Windows startup registration (`HKCU\...\Run`)

pub mod audio;
pub mod autostart;
pub mod bluetooth;

pub use audio::{AudioEndpoint, AudioEndpointSanitizer};
pub use autostart::AutostartManager;
pub use bluetooth::{DiscoveredDevice, WinRtRfcommTransport, WindowsBluetoothManager};
