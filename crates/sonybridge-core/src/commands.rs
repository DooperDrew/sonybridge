use crate::models::{AncMode, BatteryInfo, EqPreset, EqualizerState};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CommandError {
    #[error("Unknown command type: {0:#04x}")]
    UnknownCommand(u8),
    #[error("Payload too short for command {0:#04x} (got {1} bytes)")]
    PayloadTooShort(u8, usize),
    #[error("Invalid parameter value: {0}")]
    InvalidParameter(String),
}

/// Known Sony MDR command categories.
pub mod opcodes {
    pub const CMD_HANDSHAKE: u8 = 0x00;
    pub const CMD_BATTERY: u8 = 0x02;
    pub const CMD_EQUALIZER: u8 = 0x04;
    pub const CMD_NOISE_CANCELING: u8 = 0x08;
    pub const CMD_SYSTEM_SETTINGS: u8 = 0x0A;
}

/// Build handshake payload to query protocol capabilities.
pub fn build_handshake_query() -> Vec<u8> {
    vec![opcodes::CMD_HANDSHAKE, 0x00, 0x01]
}

/// Build battery status query payload.
pub fn build_battery_query() -> Vec<u8> {
    vec![opcodes::CMD_BATTERY, 0x00]
}

/// Parse battery status response payload.
/// Expected format: [0x02, 0x01, percentage (0-100), charging_flag (0 or 1)]
pub fn parse_battery_response(payload: &[u8]) -> Result<BatteryInfo, CommandError> {
    if payload.len() < 3 {
        return Err(CommandError::PayloadTooShort(
            opcodes::CMD_BATTERY,
            payload.len(),
        ));
    }
    if payload[0] != opcodes::CMD_BATTERY {
        return Err(CommandError::UnknownCommand(payload[0]));
    }
    let percentage = payload[2].min(100);
    let charging = payload.get(3).map(|&b| b != 0).unwrap_or(false);
    Ok(BatteryInfo {
        percentage,
        charging,
    })
}

/// Build ANC / Ambient Sound control payload.
/// Expected format for Set ANC:
/// [0x08, 0x01, mode_byte, ambient_level, voice_focus_flag]
pub fn build_anc_set(mode: &AncMode) -> Vec<u8> {
    match mode {
        AncMode::Off => vec![opcodes::CMD_NOISE_CANCELING, 0x01, 0x00, 0x00, 0x00],
        AncMode::NoiseCanceling => vec![opcodes::CMD_NOISE_CANCELING, 0x01, 0x01, 0x00, 0x00],
        AncMode::WindNoiseReduction => vec![opcodes::CMD_NOISE_CANCELING, 0x01, 0x03, 0x00, 0x00],
        AncMode::AmbientSound {
            level,
            focus_on_voice,
        } => {
            let clamped_level = (*level).clamp(1, 20);
            let focus_byte = if *focus_on_voice { 0x01 } else { 0x00 };
            vec![
                opcodes::CMD_NOISE_CANCELING,
                0x01,
                0x02,
                clamped_level,
                focus_byte,
            ]
        }
    }
}

/// Parse ANC / Ambient Sound response or notification payload.
pub fn parse_anc_response(payload: &[u8]) -> Result<AncMode, CommandError> {
    if payload.len() < 3 {
        return Err(CommandError::PayloadTooShort(
            opcodes::CMD_NOISE_CANCELING,
            payload.len(),
        ));
    }
    if payload[0] != opcodes::CMD_NOISE_CANCELING {
        return Err(CommandError::UnknownCommand(payload[0]));
    }

    let mode_byte = payload[2];
    match mode_byte {
        0x00 => Ok(AncMode::Off),
        0x01 => Ok(AncMode::NoiseCanceling),
        0x03 => Ok(AncMode::WindNoiseReduction),
        0x02 => {
            let level = payload.get(3).copied().unwrap_or(10).clamp(1, 20);
            let focus = payload.get(4).map(|&b| b != 0).unwrap_or(false);
            Ok(AncMode::AmbientSound {
                level,
                focus_on_voice: focus,
            })
        }
        other => Err(CommandError::InvalidParameter(format!(
            "unknown ANC mode byte {other:#04x}"
        ))),
    }
}

/// Build Equalizer command payload.
/// Format: [0x04, 0x01, preset_id, band0, band1, band2, band3, band4, clear_bass]
pub fn build_equalizer_set(state: &EqualizerState) -> Vec<u8> {
    let preset_id = match state.preset {
        EqPreset::Off => 0x00,
        EqPreset::Bright => 0x01,
        EqPreset::Excited => 0x02,
        EqPreset::Mellow => 0x03,
        EqPreset::Relaxed => 0x04,
        EqPreset::Vocal => 0x05,
        EqPreset::TrebleBoost => 0x06,
        EqPreset::BassBoost => 0x07,
        EqPreset::Speech => 0x08,
        EqPreset::Manual => 0x09,
        EqPreset::Custom1 => 0x0A,
        EqPreset::Custom2 => 0x0B,
    };

    let mut payload = vec![opcodes::CMD_EQUALIZER, 0x01, preset_id];
    // Map -10..+10 dB to unsigned byte representation (offset by +10)
    for band in state.bands {
        let clamped = band.clamp(-10, 10);
        payload.push((clamped + 10) as u8);
    }
    let clear_bass_clamped = state.clear_bass.clamp(-10, 10);
    payload.push((clear_bass_clamped + 10) as u8);

    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_response_parsing() {
        let payload = vec![0x02, 0x01, 85, 0x00];
        let info = parse_battery_response(&payload).unwrap();
        assert_eq!(info.percentage, 85);
        assert!(!info.charging);

        let charging_payload = vec![0x02, 0x01, 100, 0x01];
        let charging_info = parse_battery_response(&charging_payload).unwrap();
        assert_eq!(charging_info.percentage, 100);
        assert!(charging_info.charging);
    }

    #[test]
    fn test_anc_set_and_parse_roundtrip() {
        let anc_on = AncMode::NoiseCanceling;
        let bytes = build_anc_set(&anc_on);
        assert_eq!(bytes, vec![0x08, 0x01, 0x01, 0x00, 0x00]);

        let parsed = parse_anc_response(&bytes).unwrap();
        assert_eq!(parsed, AncMode::NoiseCanceling);

        let ambient = AncMode::AmbientSound {
            level: 14,
            focus_on_voice: true,
        };
        let ambient_bytes = build_anc_set(&ambient);
        assert_eq!(ambient_bytes, vec![0x08, 0x01, 0x02, 14, 0x01]);

        let parsed_ambient = parse_anc_response(&ambient_bytes).unwrap();
        assert_eq!(parsed_ambient, ambient);
    }

    #[test]
    fn test_equalizer_set_encoding() {
        let eq = EqualizerState {
            preset: EqPreset::Excited,
            bands: [-2, 0, 3, 5, 2],
            clear_bass: 4,
        };
        let bytes = build_equalizer_set(&eq);
        assert_eq!(bytes[0], 0x04);
        assert_eq!(bytes[1], 0x01);
        assert_eq!(bytes[2], 0x02); // Excited preset
                                    // -2 + 10 = 8, 0 + 10 = 10, 3 + 10 = 13, 5 + 10 = 15, 2 + 10 = 12
        assert_eq!(&bytes[3..8], &[8, 10, 13, 15, 12]);
        // 4 + 10 = 14
        assert_eq!(bytes[8], 14);
    }
}
