use crate::models::{AncMode, EqPreset, EqualizerState};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A saved acoustic & ANC profile for quick 1-click switching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoundProfile {
    pub name: String,
    pub anc: AncMode,
    pub equalizer: EqualizerState,
}

/// Persistent user configuration for SonyBridge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub language: String,
    pub auto_connect_on_startup: bool,
    pub enforce_a2dp_on_connect: bool,
    pub low_battery_threshold: u8,
    pub onboarding_completed: bool,
    pub active_profile: String,
    pub profiles: Vec<SoundProfile>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            auto_connect_on_startup: true,
            enforce_a2dp_on_connect: true,
            low_battery_threshold: 20,
            onboarding_completed: false,
            active_profile: "Focus (ANC + Excited)".to_string(),
            profiles: vec![
                SoundProfile {
                    name: "Focus (ANC + Excited)".to_string(),
                    anc: AncMode::NoiseCanceling,
                    equalizer: EqualizerState {
                        preset: EqPreset::Excited,
                        bands: [-1, 2, 4, 3, 1],
                        clear_bass: 3,
                    },
                },
                SoundProfile {
                    name: "Office / Calls (Ambient + Vocal)".to_string(),
                    anc: AncMode::AmbientSound {
                        level: 12,
                        focus_on_voice: true,
                    },
                    equalizer: EqualizerState {
                        preset: EqPreset::Vocal,
                        bands: [-2, 3, 5, 2, -1],
                        clear_bass: -1,
                    },
                },
                SoundProfile {
                    name: "Cinema / Bass Boost".to_string(),
                    anc: AncMode::NoiseCanceling,
                    equalizer: EqualizerState {
                        preset: EqPreset::BassBoost,
                        bands: [4, 2, 0, 0, 0],
                        clear_bass: 6,
                    },
                },
                SoundProfile {
                    name: "Reference (Flat + ANC Off)".to_string(),
                    anc: AncMode::Off,
                    equalizer: EqualizerState::default(),
                },
            ],
        }
    }
}

impl AppConfig {
    /// Load configuration from a JSON file, or return `AppConfig::default()` if missing/invalid.
    pub fn load_from_path(path: &Path) -> Self {
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(cfg) = serde_json::from_str::<Self>(&contents) {
                return cfg;
            }
        }
        Self::default()
    }

    /// Save configuration to a JSON file, creating parent directories if needed.
    pub fn save_to_path(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let serialized = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, serialized).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_profiles() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.language, "en");
        assert!(cfg.auto_connect_on_startup);
        assert!(cfg.enforce_a2dp_on_connect);
        assert_eq!(cfg.profiles.len(), 4);
    }

    #[test]
    fn test_config_json_roundtrip() {
        let cfg = AppConfig {
            language: "it".to_string(),
            onboarding_completed: true,
            ..Default::default()
        };

        let json = serde_json::to_string(&cfg).unwrap();
        let decoded: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, cfg);
    }
}
