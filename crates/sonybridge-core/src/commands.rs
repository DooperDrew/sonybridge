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

/// Known Sony MDR command categories (supporting both canonical and WH-1000XM3 MDR v1 opcodes).
pub mod opcodes {
    pub const CMD_HANDSHAKE: u8 = 0x00;
    pub const CMD_HANDSHAKE_RET: u8 = 0x01;
    pub const CMD_BATTERY: u8 = 0x02;
    pub const CMD_EQUALIZER: u8 = 0x04;
    pub const CMD_NOISE_CANCELING: u8 = 0x08;
    pub const CMD_SYSTEM_SETTINGS: u8 = 0x0A;

    // Physical WH-1000XM3 / MDR v1 hardware opcodes
    pub const V1_BATTERY_GET: u8 = 0x12;
    pub const V1_BATTERY_RET: u8 = 0x13;
    pub const V1_BATTERY_NOTIFY: u8 = 0x15;
    pub const V1_POWER_AUTO_OFF_SET: u8 = 0x28;
    pub const V1_EQEBB_GET: u8 = 0x56;
    pub const V1_EQEBB_RET: u8 = 0x57;
    pub const V1_EQEBB_SET: u8 = 0x58;
    pub const V1_EQEBB_NOTIFY: u8 = 0x59;
    pub const V1_NCASM_GET: u8 = 0x66;
    pub const V1_NCASM_RET: u8 = 0x67;
    pub const V1_NCASM_SET: u8 = 0x68;
    pub const V1_NCASM_NOTIFY: u8 = 0x69;
    pub const V1_SYSTEM_SET_PARAM: u8 = 0xD8;
}

/// Build handshake payload to query protocol capabilities.
pub fn build_handshake_query() -> Vec<u8> {
    vec![opcodes::CMD_HANDSHAKE, 0x00]
}

/// Build battery status query payload.
pub fn build_battery_query() -> Vec<u8> {
    vec![opcodes::CMD_BATTERY, 0x00]
}

/// Build physical WH-1000XM3 MDR v1 battery inquiry (`0x12, 0x00`).
pub fn build_v1_battery_query() -> Vec<u8> {
    vec![opcodes::V1_BATTERY_GET, 0x00]
}

/// Parse battery status response payload (supports both canonical `0x02` and MDR v1 `0x13`/`0x15`).
pub fn parse_battery_response(payload: &[u8]) -> Result<BatteryInfo, CommandError> {
    if payload.len() < 3 {
        return Err(CommandError::PayloadTooShort(
            payload.first().copied().unwrap_or(opcodes::CMD_BATTERY),
            payload.len(),
        ));
    }
    match payload[0] {
        opcodes::CMD_BATTERY
        | opcodes::V1_BATTERY_GET
        | opcodes::V1_BATTERY_RET
        | opcodes::V1_BATTERY_NOTIFY => {
            let percentage = payload[2].min(100);
            let charging = payload.get(3).map(|&b| b != 0).unwrap_or(false);
            Ok(BatteryInfo {
                percentage,
                charging,
            })
        }
        other => Err(CommandError::UnknownCommand(other)),
    }
}

/// Build ANC / Ambient Sound control payload (canonical format).
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

/// Build physical WH-1000XM3 MDR v1 `NCASM_SET_PARAM` (`0x68`) payload.
/// Format: `[0x68, 0x02, nc_asm_effect, nc_asm_setting_type, nc_dual_single_value, asm_setting_type, asm_id, asm_level]`
pub fn build_v1_ncasm_set(mode: &AncMode) -> Vec<u8> {
    match mode {
        AncMode::Off => vec![
            opcodes::V1_NCASM_SET,
            0x02,
            0x00, // Effect: Off
            0x01,
            0x00,
            0x01,
            0x00,
            0x00,
        ],
        AncMode::NoiseCanceling => vec![
            opcodes::V1_NCASM_SET,
            0x02,
            0x01, // Effect: On
            0x01,
            0x02, // Dual NC (Full Noise Canceling)
            0x01,
            0x00,
            0x00,
        ],
        AncMode::WindNoiseReduction => vec![
            opcodes::V1_NCASM_SET,
            0x02,
            0x01, // Effect: On
            0x01,
            0x01, // Single NC (Wind Noise Reduction)
            0x01,
            0x00,
            0x01,
        ],
        AncMode::AmbientSound {
            level,
            focus_on_voice,
        } => {
            let clamped_level = (*level).clamp(1, 20);
            let voice_byte = if *focus_on_voice { 0x01 } else { 0x00 };
            vec![
                opcodes::V1_NCASM_SET,
                0x02,
                0x11, // Adjustment / Ambient active
                0x01,
                0x00, // NC off, ASM active
                0x01,
                voice_byte,
                clamped_level,
            ]
        }
    }
}

/// Parse ANC / Ambient Sound response or notification payload (supports both canonical `0x08` and MDR v1 `0x67`/`0x68`/`0x69`).
pub fn parse_anc_response(payload: &[u8]) -> Result<AncMode, CommandError> {
    if payload.len() < 3 {
        return Err(CommandError::PayloadTooShort(
            payload
                .first()
                .copied()
                .unwrap_or(opcodes::CMD_NOISE_CANCELING),
            payload.len(),
        ));
    }

    match payload[0] {
        opcodes::CMD_NOISE_CANCELING => {
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
        opcodes::V1_NCASM_RET | opcodes::V1_NCASM_SET | opcodes::V1_NCASM_NOTIFY => {
            if payload.len() < 8 {
                return Err(CommandError::PayloadTooShort(payload[0], payload.len()));
            }
            let effect = payload[2];
            let nc_value = payload[4];
            let voice_focus = payload[6] != 0;
            let asm_level = payload[7];

            if effect == 0x00 {
                Ok(AncMode::Off)
            } else if nc_value == 0x02 && asm_level == 0 {
                Ok(AncMode::NoiseCanceling)
            } else if nc_value == 0x01 && asm_level <= 1 {
                Ok(AncMode::WindNoiseReduction)
            } else {
                Ok(AncMode::AmbientSound {
                    level: asm_level.clamp(1, 20),
                    focus_on_voice: voice_focus,
                })
            }
        }
        other => Err(CommandError::UnknownCommand(other)),
    }
}

/// Build Equalizer command payload (canonical format).
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
    for band in state.bands {
        let clamped = band.clamp(-10, 10);
        payload.push((clamped + 10) as u8);
    }
    let clear_bass_clamped = state.clear_bass.clamp(-10, 10);
    payload.push((clear_bass_clamped + 10) as u8);

    payload
}

/// Build physical WH-1000XM3 MDR v1 `EQEBB_SET_PARAM` (`0x58`) payload.
/// Format: `[0x58, 0x00, preset_id, 0x06, clear_bass+10, band0+10, band1+10, band2+10, band3+10, band4+10]`
pub fn build_v1_equalizer_set(state: &EqualizerState) -> Vec<u8> {
    let v1_preset_id = match state.preset {
        EqPreset::Off => 0x00,
        EqPreset::Bright => 0x10,
        EqPreset::Excited => 0x11,
        EqPreset::Mellow => 0x12,
        EqPreset::Relaxed => 0x13,
        EqPreset::Vocal => 0x14,
        EqPreset::TrebleBoost => 0x15,
        EqPreset::BassBoost => 0x16,
        EqPreset::Speech => 0x17,
        EqPreset::Manual => 0xA0,
        EqPreset::Custom1 => 0xA1,
        EqPreset::Custom2 => 0xA2,
    };

    let clear_bass = (state.clear_bass.clamp(-10, 10) + 10) as u8;
    let mut payload = vec![
        opcodes::V1_EQEBB_SET,
        0x00,
        v1_preset_id,
        0x06, // 1 Clear Bass + 5 EQ bands
        clear_bass,
    ];
    for band in state.bands {
        payload.push((band.clamp(-10, 10) + 10) as u8);
    }
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

        // V1 hardware response
        let v1_payload = vec![0x13, 0x00, 70, 0x01];
        let v1_info = parse_battery_response(&v1_payload).unwrap();
        assert_eq!(v1_info.percentage, 70);
        assert!(v1_info.charging);
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

        // V1 hardware NCASM roundtrip
        let v1_bytes = build_v1_ncasm_set(&ambient);
        let parsed_v1 = parse_anc_response(&v1_bytes).unwrap();
        assert_eq!(parsed_v1, ambient);
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
        assert_eq!(&bytes[3..8], &[8, 10, 13, 15, 12]);
        assert_eq!(bytes[8], 14);

        let v1_eq = build_v1_equalizer_set(&eq);
        assert_eq!(v1_eq[0], 0x58);
        assert_eq!(v1_eq[2], 0x11); // V1 Excited
        assert_eq!(v1_eq[4], 14); // Clear Bass +4 -> 14
    }
}
