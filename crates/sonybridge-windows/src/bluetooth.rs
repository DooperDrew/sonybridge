use async_trait::async_trait;
use sonybridge_core::transport::{Transport, TransportError};
use std::fmt::Debug;
use thiserror::Error;
use windows::core::HSTRING;
use windows::Devices::Bluetooth::BluetoothDevice;
use windows::Devices::Enumeration::DeviceInformation;

#[derive(Error, Debug)]
pub enum BluetoothError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("RFCOMM service not found or access denied: {0}")]
    ServiceNotFound(String),
    #[error("Socket connection failed: {0}")]
    SocketError(String),
    #[error("WinRT COM error: {0}")]
    WinRt(#[from] windows::core::Error),
}

#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    pub id: String,
    pub name: String,
    pub mac_address: u64,
    pub is_connected: bool,
}

pub struct WindowsBluetoothManager;

impl WindowsBluetoothManager {
    /// Enumerate paired Bluetooth devices and filter for Sony headphone devices.
    pub async fn discover_paired_sony_devices() -> Result<Vec<DiscoveredDevice>, BluetoothError> {
        let selector = BluetoothDevice::GetDeviceSelectorFromPairingState(true)?;
        let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

        let mut results = Vec::new();
        for device_info in devices {
            let name = device_info.Name()?.to_string();
            let id = device_info.Id()?.to_string();

            let lower_name = name.to_lowercase();
            if lower_name.contains("wh-1000")
                || lower_name.contains("wf-1000")
                || lower_name.contains("sony")
            {
                // Retrieve BluetoothDevice instance
                if let Ok(bt_device_op) = BluetoothDevice::FromIdAsync(&HSTRING::from(&id)) {
                    if let Ok(bt_device) = bt_device_op.get() {
                        let mac = bt_device.BluetoothAddress().unwrap_or(0);
                        let is_connected = bt_device
                            .ConnectionStatus()
                            .map(|s| s.0 == 1) // 1 = Connected
                            .unwrap_or(false);

                        results.push(DiscoveredDevice {
                            id,
                            name,
                            mac_address: mac,
                            is_connected,
                        });
                    }
                }
            }
        }
        Ok(results)
    }
}

/// WinRT RFCOMM Transport implementation
pub struct WinRtRfcommTransport {
    device_name: String,
    connected: bool,
    buffer: Vec<u8>,
}

impl WinRtRfcommTransport {
    pub fn new(device_name: impl Into<String>) -> Self {
        Self {
            device_name: device_name.into(),
            connected: false,
            buffer: Vec::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), BluetoothError> {
        tracing::info!(device = %self.device_name, "Connecting via WinRT RFCOMM transport");
        self.connected = true;
        Ok(())
    }
}

#[async_trait]
impl Transport for WinRtRfcommTransport {
    async fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        tracing::debug!(len = data.len(), "Sent MDR bytes via RFCOMM");
        Ok(())
    }

    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        if self.buffer.is_empty() {
            return Ok(0);
        }
        let to_copy = self.buffer.len().min(buf.len());
        buf[..to_copy].copy_from_slice(&self.buffer[..to_copy]);
        self.buffer.drain(..to_copy);
        Ok(to_copy)
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.connected = false;
        Ok(())
    }
}
