use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Device not connected")]
    NotConnected,
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Connection timed out")]
    Timeout,
    #[error("Device disconnected")]
    Disconnected,
}

/// Abstract asynchronous bidirectional transport for Sony MDR protocol frames.
#[async_trait::async_trait]
pub trait Transport: Send + Sync {
    /// Send raw bytes to the headphone control channel.
    async fn send(&mut self, data: &[u8]) -> Result<(), TransportError>;

    /// Read raw bytes into buffer from the headphone control channel.
    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize, TransportError>;

    /// Check if transport is currently connected.
    fn is_connected(&self) -> bool;

    /// Disconnect transport.
    async fn disconnect(&mut self) -> Result<(), TransportError>;
}

/// Mock transport for headless testing and invariant validation.
#[derive(Debug, Default)]
pub struct MockTransport {
    pub connected: bool,
    pub sent_packets: Vec<Vec<u8>>,
    pub incoming_packets: Vec<Vec<u8>>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            connected: true,
            sent_packets: Vec::new(),
            incoming_packets: Vec::new(),
        }
    }

    pub fn queue_incoming(&mut self, data: Vec<u8>) {
        self.incoming_packets.push(data);
    }
}

#[async_trait::async_trait]
impl Transport for MockTransport {
    async fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        self.sent_packets.push(data.to_vec());
        Ok(())
    }

    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        if self.incoming_packets.is_empty() {
            return Ok(0);
        }
        let packet = self.incoming_packets.remove(0);
        let len = packet.len().min(buf.len());
        buf[..len].copy_from_slice(&packet[..len]);
        Ok(len)
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_transport_send_receive() {
        let mut transport = MockTransport::new();
        assert!(transport.is_connected());

        transport.send(&[0x0C, 0x01, 0x00]).await.unwrap();
        assert_eq!(transport.sent_packets.len(), 1);
        assert_eq!(transport.sent_packets[0], vec![0x0C, 0x01, 0x00]);

        transport.queue_incoming(vec![0x0C, 0x02, 0xFF]);
        let mut buf = [0u8; 16];
        let bytes_read = transport.receive(&mut buf).await.unwrap();
        assert_eq!(bytes_read, 3);
        assert_eq!(&buf[..bytes_read], &[0x0C, 0x02, 0xFF]);

        transport.disconnect().await.unwrap();
        assert!(!transport.is_connected());
        assert!(transport.send(&[0x00]).await.is_err());
    }
}
