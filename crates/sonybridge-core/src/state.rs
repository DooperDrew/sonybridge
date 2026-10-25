use crate::commands::{self, opcodes};
use crate::models::{AncMode, BatteryInfo, DeviceSettings, EqualizerState, HeadphoneModel};
use crate::protocol::MdrFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Handshaking,
    Connected,
    Degraded(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadphoneState {
    pub status: ConnectionStatus,
    pub model: Option<HeadphoneModel>,
    pub device_name: Option<String>,
    pub mac_address: Option<String>,
    pub battery: BatteryInfo,
    pub anc: AncMode,
    pub equalizer: EqualizerState,
    pub settings: DeviceSettings,
}

impl Default for HeadphoneState {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            model: None,
            device_name: None,
            mac_address: None,
            battery: BatteryInfo::default(),
            anc: AncMode::default(),
            equalizer: EqualizerState::default(),
            settings: DeviceSettings::default(),
        }
    }
}

pub struct StateMachine {
    pub state: HeadphoneState,
    seq_counter: u8,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: HeadphoneState::default(),
            seq_counter: 0,
        }
    }

    pub fn next_seq(&mut self) -> u8 {
        let seq = self.seq_counter;
        self.seq_counter = self.seq_counter.wrapping_add(1);
        seq
    }

    /// Process incoming parsed MDR frame and update state accordingly.
    pub fn handle_incoming_frame(&mut self, frame: &MdrFrame) -> Result<(), String> {
        if frame.payload.is_empty() {
            return Err("Empty payload in MDR frame".to_string());
        }

        let cmd = frame.payload[0];
        match cmd {
            opcodes::CMD_BATTERY | opcodes::V1_BATTERY_RET | opcodes::V1_BATTERY_NOTIFY => {
                if let Ok(battery) = commands::parse_battery_response(&frame.payload) {
                    self.state.battery = battery;
                }
            }
            opcodes::CMD_NOISE_CANCELING
            | opcodes::V1_NCASM_RET
            | opcodes::V1_NCASM_SET
            | opcodes::V1_NCASM_NOTIFY => {
                if let Ok(anc) = commands::parse_anc_response(&frame.payload) {
                    self.state.anc = anc;
                }
            }
            _ => {
                // Ignore unrecognized command bytes gracefully
            }
        }
        Ok(())
    }

    /// Create an outgoing MDR frame for setting ANC mode (canonical).
    pub fn set_anc(&mut self, mode: AncMode) -> MdrFrame {
        self.state.anc = mode.clone();
        let payload = commands::build_anc_set(&mode);
        let seq = self.next_seq();
        MdrFrame::new(seq, payload)
    }

    /// Create an outgoing WH-1000XM3 MDR v1 hardware frame for setting ANC/ASM mode.
    pub fn set_v1_anc(&mut self, mode: AncMode) -> MdrFrame {
        self.state.anc = mode.clone();
        let payload = commands::build_v1_ncasm_set(&mode);
        let seq = self.next_seq();
        MdrFrame::new(seq, payload)
    }

    /// Create an outgoing MDR frame for setting Equalizer state (canonical).
    pub fn set_equalizer(&mut self, eq: EqualizerState) -> MdrFrame {
        self.state.equalizer = eq.clone();
        let payload = commands::build_equalizer_set(&eq);
        let seq = self.next_seq();
        MdrFrame::new(seq, payload)
    }

    /// Create an outgoing WH-1000XM3 MDR v1 hardware frame for setting Equalizer state.
    pub fn set_v1_equalizer(&mut self, eq: EqualizerState) -> MdrFrame {
        self.state.equalizer = eq.clone();
        let payload = commands::build_v1_equalizer_set(&eq);
        let seq = self.next_seq();
        MdrFrame::new(seq, payload)
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_anc_transition() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.state.anc, AncMode::NoiseCanceling);

        let frame = sm.set_anc(AncMode::Off);
        assert_eq!(sm.state.anc, AncMode::Off);
        assert_eq!(frame.payload, vec![0x08, 0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_incoming_battery_notification() {
        let mut sm = StateMachine::new();
        let frame = MdrFrame::new(1, vec![0x02, 0x01, 72, 0x01]);
        sm.handle_incoming_frame(&frame).unwrap();

        assert_eq!(sm.state.battery.percentage, 72);
        assert!(sm.state.battery.charging);
    }
}
