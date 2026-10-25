use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadphoneModel {
    Wh1000Xm3,
    Wh1000Xm4,
    Wh1000Xm5,
    Wf1000Xm4,
    Wf1000Xm5,
    Unknown(String),
}

impl HeadphoneModel {
    pub fn from_device_name(name: &str) -> Self {
        let lower = name.to_lowercase();
        if lower.contains("wh-1000xm3") {
            Self::Wh1000Xm3
        } else if lower.contains("wh-1000xm4") {
            Self::Wh1000Xm4
        } else if lower.contains("wh-1000xm5") {
            Self::Wh1000Xm5
        } else if lower.contains("wf-1000xm4") {
            Self::Wf1000Xm4
        } else if lower.contains("wf-1000xm5") {
            Self::Wf1000Xm5
        } else {
            Self::Unknown(name.to_string())
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Wh1000Xm3 => "Sony WH-1000XM3",
            Self::Wh1000Xm4 => "Sony WH-1000XM4",
            Self::Wh1000Xm5 => "Sony WH-1000XM5",
            Self::Wf1000Xm4 => "Sony WF-1000XM4",
            Self::Wf1000Xm5 => "Sony WF-1000XM5",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub charging: bool,
}

impl Default for BatteryInfo {
    fn default() -> Self {
        Self {
            percentage: 100,
            charging: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AncMode {
    Off,
    #[default]
    NoiseCanceling,
    AmbientSound {
        /// Ambient level from 1 (minimum transparency) to 20 (maximum transparency)
        level: u8,
        /// Emphasize human voice frequencies
        focus_on_voice: bool,
    },
    WindNoiseReduction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqPreset {
    Off,
    Bright,
    Excited,
    Mellow,
    Relaxed,
    Vocal,
    TrebleBoost,
    BassBoost,
    Speech,
    Manual,
    Custom1,
    Custom2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EqualizerState {
    pub preset: EqPreset,
    /// 5 frequency bands: [400Hz, 1kHz, 2.5kHz, 6.3kHz, 16kHz]
    /// Value range: -10 to +10 dB
    pub bands: [i8; 5],
    /// Clear Bass level: -10 to +10
    pub clear_bass: i8,
}

impl Default for EqualizerState {
    fn default() -> Self {
        Self {
            preset: EqPreset::Off,
            bands: [0, 0, 0, 0, 0],
            clear_bass: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSettings {
    pub auto_power_off_minutes: Option<u32>,
    pub touch_sensor_enabled: bool,
    pub voice_guidance_enabled: bool,
    pub sound_position: u8,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            auto_power_off_minutes: Some(30),
            touch_sensor_enabled: true,
            voice_guidance_enabled: true,
            sound_position: 0,
        }
    }
}
