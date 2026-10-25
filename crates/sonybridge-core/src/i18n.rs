use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleBundle {
    pub code: String,
    pub strings: HashMap<String, String>,
}

impl LocaleBundle {
    pub fn english() -> Self {
        let mut strings = HashMap::new();
        strings.insert("app_title".into(), "SonyBridge".into());
        strings.insert("subtitle".into(), "WH-1000XM3 / XM4 / XM5 Companion".into());
        strings.insert("status_connected".into(), "Connected".into());
        strings.insert("status_disconnected".into(), "Disconnected".into());
        strings.insert("btn_reconnect".into(), "Reconnect".into());
        strings.insert("tab_dashboard".into(), "Dashboard".into());
        strings.insert("tab_profiles".into(), "Profiles & EQ".into());
        strings.insert("tab_audio".into(), "Windows & Audio".into());
        strings.insert("tab_architecture".into(), "Architecture".into());
        strings.insert("anc_title".into(), "Noise Canceling & Ambient Sound".into());
        strings.insert("mode_nc".into(), "Noise Canceling".into());
        strings.insert("mode_ambient".into(), "Ambient Sound".into());
        strings.insert("mode_off".into(), "Off".into());
        strings.insert("voice_focus".into(), "Focus on Voice".into());
        strings.insert("eq_title".into(), "5-Band Hardware Equalizer".into());
        strings.insert("clear_bass".into(), "Clear Bass (DSP Sub-Bass)".into());
        strings.insert("enforce_a2dp".into(), "Enforce Stereo A2DP".into());
        Self {
            code: "en".to_string(),
            strings,
        }
    }

    pub fn italian() -> Self {
        let mut strings = HashMap::new();
        strings.insert("app_title".into(), "SonyBridge".into());
        strings.insert(
            "subtitle".into(),
            "Assistente WH-1000XM3 / XM4 / XM5".into(),
        );
        strings.insert("status_connected".into(), "Connesso".into());
        strings.insert("status_disconnected".into(), "Disconnesso".into());
        strings.insert("btn_reconnect".into(), "Riconnetti".into());
        strings.insert("tab_dashboard".into(), "Pannello".into());
        strings.insert("tab_profiles".into(), "Profili & EQ".into());
        strings.insert("tab_audio".into(), "Windows & Audio".into());
        strings.insert("tab_architecture".into(), "Architettura".into());
        strings.insert(
            "anc_title".into(),
            "Eliminazione Rumore & Suono Ambiente".into(),
        );
        strings.insert("mode_nc".into(), "Eliminazione Rumore".into());
        strings.insert("mode_ambient".into(), "Suono Ambiente".into());
        strings.insert("mode_off".into(), "Disattivato".into());
        strings.insert("voice_focus".into(), "Risalta Voce".into());
        strings.insert("eq_title".into(), "Equalizzatore Hardware a 5 Bande".into());
        strings.insert("clear_bass".into(), "Clear Bass (Sub-Bassi DSP)".into());
        strings.insert("enforce_a2dp".into(), "Forza Stereo A2DP".into());
        Self {
            code: "it".to_string(),
            strings,
        }
    }

    pub fn for_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "it" | "it-it" => Self::italian(),
            _ => Self::english(),
        }
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_bundles() {
        let en = LocaleBundle::for_code("en");
        let it = LocaleBundle::for_code("it");
        assert_eq!(en.get("status_connected"), "Connected");
        assert_eq!(it.get("status_connected"), "Connesso");
    }
}
