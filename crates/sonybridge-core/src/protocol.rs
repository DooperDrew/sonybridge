use thiserror::Error;

pub const START_MARKER_NORMAL: u8 = 0x0C;
pub const START_MARKER_EXTENDED: u8 = 0x0E;
pub const ESCAPE_MARKER: u8 = 0x0D;

/// Real-world Sony MDR v1/v2 RFCOMM wire markers (WH-1000XM3 / WH-1000XM4)
pub const WIRE_START_MARKER: u8 = 0x3E; // '>'
pub const WIRE_END_MARKER: u8 = 0x3C; // '<'
pub const WIRE_ESCAPE_BYTE: u8 = 0x3D; // '='
pub const WIRE_ESCAPE_MASK: u8 = 0x10;

pub const DATA_TYPE_DATA: u8 = 0x00;
pub const DATA_TYPE_ACK: u8 = 0x01;
pub const DATA_TYPE_MDR: u8 = 0x0C;
pub const DATA_TYPE_MDR_NO2: u8 = 0x0E;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FrameError {
    #[error("Frame too short (got {0} bytes, minimum 7)")]
    TooShort(usize),
    #[error("Invalid start byte: {0:#04x} (expected 0x0C, 0x0E, or 0x3E)")]
    InvalidStartByte(u8),
    #[error("Invalid end byte: {0:#04x} (expected 0x3C)")]
    InvalidEndByte(u8),
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

    /// Create an ACK frame for the Sony MDR protocol (`data_type = 0x01`, empty payload).
    pub fn new_ack(seq_no: u8) -> Self {
        Self {
            start_byte: DATA_TYPE_ACK,
            seq_no,
            payload: Vec::new(),
            checksum: DATA_TYPE_ACK.wrapping_add(seq_no),
        }
    }

    /// Calculate the Sony MDR 8-bit checksum: sum of all payload bytes mod 256.
    pub fn calculate_checksum(payload: &[u8]) -> u8 {
        let sum: u32 = payload.iter().map(|&b| b as u32).sum();
        (sum & 0xFF) as u8
    }

    /// Calculate the full Sony MDR v1 wire checksum:
    /// `(data_type + seq_no + sum(len_be_bytes) + sum(payload)) & 0xFF`
    pub fn calculate_wire_checksum(data_type: u8, seq_no: u8, payload: &[u8]) -> u8 {
        let len_bytes = (payload.len() as u32).to_be_bytes();
        let mut sum: u32 = (data_type as u32) + (seq_no as u32);
        for &b in &len_bytes {
            sum += b as u32;
        }
        for &b in payload {
            sum += b as u32;
        }
        (sum & 0xFF) as u8
    }

    /// Escape reserved wire bytes (`0x3C`, `0x3D`, `0x3E`) using `0x3D` + (`byte & !0x10`).
    pub fn escape_wire_bytes(raw: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(raw.len() + 4);
        for &b in raw {
            if b == WIRE_START_MARKER || b == WIRE_END_MARKER || b == WIRE_ESCAPE_BYTE {
                out.push(WIRE_ESCAPE_BYTE);
                out.push(b & !WIRE_ESCAPE_MASK);
            } else {
                out.push(b);
            }
        }
        out
    }

    /// Unescape byte-stuffed wire payload between `0x3E` and `0x3C`.
    pub fn unescape_wire_bytes(escaped: &[u8]) -> Result<Vec<u8>, FrameError> {
        let mut out = Vec::with_capacity(escaped.len());
        let mut i = 0;
        while i < escaped.len() {
            if escaped[i] == WIRE_ESCAPE_BYTE {
                if i + 1 >= escaped.len() {
                    return Err(FrameError::MalformedPayload(
                        "Trailing escape byte 0x3D at end of frame".to_string(),
                    ));
                }
                out.push(escaped[i + 1] | WIRE_ESCAPE_MASK);
                i += 2;
            } else {
                out.push(escaped[i]);
                i += 1;
            }
        }
        Ok(out)
    }

    /// Encode frame into raw byte buffer (direct unstuffed format).
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

    /// Encode frame into Sony WH-1000XM3 MDR v1 stuffed RFCOMM packet:
    /// `[0x3E] + escaped([DATA_TYPE, SEQ, LEN_BE4, PAYLOAD..., WIRE_CHECKSUM]) + [0x3C]`
    pub fn encode_wire(&self) -> Vec<u8> {
        let data_type = self.start_byte;
        let payload_len = self.payload.len() as u32;
        let wire_checksum = Self::calculate_wire_checksum(data_type, self.seq_no, &self.payload);

        let mut inner = Vec::with_capacity(1 + 1 + 4 + self.payload.len() + 1);
        inner.push(data_type);
        inner.push(self.seq_no);
        inner.extend_from_slice(&payload_len.to_be_bytes());
        inner.extend_from_slice(&self.payload);
        inner.push(wire_checksum);

        let escaped = Self::escape_wire_bytes(&inner);
        let mut packet = Vec::with_capacity(escaped.len() + 2);
        packet.push(WIRE_START_MARKER);
        packet.extend_from_slice(&escaped);
        packet.push(WIRE_END_MARKER);
        packet
    }

    /// Decode a complete MDR frame from either stuffed wire format (`0x3E ... 0x3C`)
    /// or direct unstuffed format (`0x0C ...`).
    pub fn decode(bytes: &[u8]) -> Result<Self, FrameError> {
        if bytes.first() == Some(&WIRE_START_MARKER) {
            return Self::decode_wire(bytes);
        }

        // Minimum valid frame: Start(1) + Seq(1) + Len(4) + Checksum(1) = 7 bytes
        if bytes.len() < 7 {
            return Err(FrameError::TooShort(bytes.len()));
        }

        let start_byte = bytes[0];
        if start_byte != START_MARKER_NORMAL
            && start_byte != START_MARKER_EXTENDED
            && start_byte != DATA_TYPE_ACK
            && start_byte != DATA_TYPE_DATA
        {
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
        let wire_checksum = Self::calculate_wire_checksum(start_byte, seq_no, &payload);

        if computed_checksum != received_checksum && wire_checksum != received_checksum {
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

    /// Decode a Sony WH-1000XM3 stuffed RFCOMM packet (`0x3E ... 0x3C`).
    pub fn decode_wire(bytes: &[u8]) -> Result<Self, FrameError> {
        if bytes.len() < 9 {
            return Err(FrameError::TooShort(bytes.len()));
        }
        if bytes[0] != WIRE_START_MARKER {
            return Err(FrameError::InvalidStartByte(bytes[0]));
        }
        if *bytes.last().unwrap() != WIRE_END_MARKER {
            return Err(FrameError::InvalidEndByte(*bytes.last().unwrap()));
        }

        let unescaped = Self::unescape_wire_bytes(&bytes[1..bytes.len() - 1])?;
        if unescaped.len() < 7 {
            return Err(FrameError::TooShort(unescaped.len()));
        }

        let data_type = unescaped[0];
        let seq_no = unescaped[1];
        let payload_len =
            u32::from_be_bytes([unescaped[2], unescaped[3], unescaped[4], unescaped[5]]) as usize;

        if unescaped.len() < 6 + payload_len + 1 {
            return Err(FrameError::LengthMismatch {
                declared: payload_len,
                actual: unescaped.len().saturating_sub(7),
            });
        }

        let payload = unescaped[6..6 + payload_len].to_vec();
        let received_checksum = unescaped[6 + payload_len];
        let computed_wire = Self::calculate_wire_checksum(data_type, seq_no, &payload);

        if computed_wire != received_checksum {
            return Err(FrameError::ChecksumMismatch {
                computed: computed_wire,
                received: received_checksum,
            });
        }

        Ok(Self {
            start_byte: data_type,
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
    fn test_wire_stuffed_roundtrip_with_reserved_bytes() {
        // Payload containing reserved markers 0x3C, 0x3D, 0x3E
        let payload = vec![0x68, 0x02, 0x3C, 0x3D, 0x3E, 0x10];
        let frame = MdrFrame::new(7, payload.clone());
        let wire_bytes = frame.encode_wire();

        assert_eq!(wire_bytes[0], WIRE_START_MARKER);
        assert_eq!(*wire_bytes.last().unwrap(), WIRE_END_MARKER);
        // No unescaped 0x3C or 0x3E inside the body
        assert!(!wire_bytes[1..wire_bytes.len() - 1].contains(&WIRE_START_MARKER));
        assert!(!wire_bytes[1..wire_bytes.len() - 1].contains(&WIRE_END_MARKER));

        let decoded = MdrFrame::decode(&wire_bytes).expect("wire frame should decode cleanly");
        assert_eq!(decoded.seq_no, 7);
        assert_eq!(decoded.payload, payload);
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
