use thiserror::Error;

pub const START_MARKER_NORMAL: u8 = 0x0C;
pub const START_MARKER_EXTENDED: u8 = 0x0E;
pub const ESCAPE_MARKER: u8 = 0x0D;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FrameError {
    #[error("Frame too short (got {0} bytes, minimum 7)")]
    TooShort(usize),
    #[error("Invalid start byte: {0:#04x} (expected 0x0C or 0x0E)")]
    InvalidStartByte(u8),
    #[error("Payload length mismatch: declared {declared}, actual buffer remaining {actual}")]
    LengthMismatch { declared: usize, actual: usize },
    #[error("Checksum mismatch: computed {computed:#04x}, received {received:#04x}")]
    ChecksumMismatch { computed: u8, received: u8 },
    #[error("Malformed payload data: {0}")]
    MalformedPayload(String),
}

/// Parsed Sony MDR frame representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdrFrame {
    pub start_byte: u8,
    pub seq_no: u8,
    pub payload: Vec<u8>,
    pub checksum: u8,
}

impl MdrFrame {
    /// Create a new frame, automatically calculating the checksum.
    pub fn new(seq_no: u8, payload: Vec<u8>) -> Self {
        let checksum = Self::calculate_checksum(&payload);
        Self {
            start_byte: START_MARKER_NORMAL,
            seq_no,
            payload,
            checksum,
        }
    }

    /// Calculate the Sony MDR 8-bit checksum: sum of all payload bytes mod 256.
    pub fn calculate_checksum(payload: &[u8]) -> u8 {
        let sum: u32 = payload.iter().map(|&b| b as u32).sum();
        (sum & 0xFF) as u8
    }

    /// Encode frame into raw byte buffer suitable for transmission over RFCOMM.
    /// Wire format:
    /// [START: 1 byte] [SEQ: 1 byte] [LEN: 4 bytes BE] [PAYLOAD: N bytes] [CHECKSUM: 1 byte]
    pub fn encode(&self) -> Vec<u8> {
        let payload_len = self.payload.len() as u32;
        let mut bytes = Vec::with_capacity(1 + 1 + 4 + self.payload.len() + 1);
        bytes.push(self.start_byte);
        bytes.push(self.seq_no);
        bytes.extend_from_slice(&payload_len.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.push(self.checksum);
        bytes
    }

    /// Decode a complete MDR frame from raw bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, FrameError> {
        // Minimum valid frame: Start(1) + Seq(1) + Len(4) + Checksum(1) = 7 bytes
        if bytes.len() < 7 {
            return Err(FrameError::TooShort(bytes.len()));
        }

        let start_byte = bytes[0];
        if start_byte != START_MARKER_NORMAL && start_byte != START_MARKER_EXTENDED {
            return Err(FrameError::InvalidStartByte(start_byte));
        }

        let seq_no = bytes[1];
        let payload_len = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]) as usize;

        let expected_total_len = 1 + 1 + 4 + payload_len + 1;
        if bytes.len() < expected_total_len {
            return Err(FrameError::LengthMismatch {
                declared: payload_len,
                actual: bytes.len().saturating_sub(6 + 1),
            });
        }

        let payload = bytes[6..6 + payload_len].to_vec();
        let received_checksum = bytes[6 + payload_len];
        let computed_checksum = Self::calculate_checksum(&payload);

        if computed_checksum != received_checksum {
            return Err(FrameError::ChecksumMismatch {
                computed: computed_checksum,
                received: received_checksum,
            });
        }

        Ok(Self {
            start_byte,
            seq_no,
            payload,
            checksum: received_checksum,
        })
    }
}

/// High-level parsed MDR packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MdrPacket {
    Raw(MdrFrame),
    HandshakeAck { protocol_version: String },
    BatteryStatus { percentage: u8, charging: bool },
    AncStatus { enabled: bool, ambient_level: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let payload = vec![0x00, 0x01, 0x02, 0x03, 0x04];
        let frame = MdrFrame::new(42, payload.clone());
        let encoded = frame.encode();

        assert_eq!(encoded[0], START_MARKER_NORMAL);
        assert_eq!(encoded[1], 42);
        assert_eq!(&encoded[2..6], &[0, 0, 0, 5]);
        assert_eq!(&encoded[6..11], &payload);
        assert_eq!(encoded[11], frame.checksum);

        let decoded = MdrFrame::decode(&encoded).expect("failed to decode valid frame");
        assert_eq!(decoded.seq_no, 42);
        assert_eq!(decoded.payload, payload);
        assert_eq!(decoded.checksum, frame.checksum);
    }

    #[test]
    fn test_checksum_verification() {
        let payload = vec![0x10, 0x20, 0x30];
        // 0x10 + 0x20 + 0x30 = 0x60
        assert_eq!(MdrFrame::calculate_checksum(&payload), 0x60);
    }

    #[test]
    fn test_corrupt_checksum_rejection() {
        let payload = vec![0x01, 0x02];
        let frame = MdrFrame::new(1, payload);
        let mut encoded = frame.encode();
        // Corrupt checksum byte
        let last_idx = encoded.len() - 1;
        encoded[last_idx] ^= 0xFF;

        let err = MdrFrame::decode(&encoded).expect_err("should reject invalid checksum");
        matches!(err, FrameError::ChecksumMismatch { .. });
    }

    #[test]
    fn test_truncated_frame_rejection() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = MdrFrame::new(1, payload);
        let encoded = frame.encode();

        let truncated = &encoded[..encoded.len() - 2];
        let err = MdrFrame::decode(truncated).expect_err("should reject truncated frame");
        matches!(err, FrameError::LengthMismatch { .. });
    }
}
