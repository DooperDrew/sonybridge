//! Windows platform-specific integrations for SonyBridge.
//!
//! Covers:
//! - Bluetooth RFCOMM / Winsock transport
//! - Audio endpoint detection and default routing (A2DP vs Hands-Free)
//! - Windows system notifications

pub mod audio;
pub mod bluetooth;

pub use audio::AudioEndpointSanitizer;
pub use bluetooth::{DiscoveredDevice, WindowsBluetoothManager};
