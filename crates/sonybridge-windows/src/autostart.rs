use std::process::Command;

const RUN_REG_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
const APP_VALUE_NAME: &str = "SonyBridge";

pub struct AutostartManager;

impl AutostartManager {
    /// Enable or disable per-user Windows login autostart for SonyBridge.
    pub fn set_autostart(enabled: bool) -> Result<(), String> {
        if enabled {
            let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
            let quoted = format!("\"{}\"", exe_path.display());
            let status = Command::new("reg")
                .args([
                    "add",
                    RUN_REG_KEY,
                    "/v",
                    APP_VALUE_NAME,
                    "/t",
                    "REG_SZ",
                    "/d",
                    &quoted,
                    "/f",
                ])
                .status()
                .map_err(|e| e.to_string())?;
            if status.success() {
                Ok(())
            } else {
                Err("reg add exited with non-zero status".to_string())
            }
        } else {
            let _ = Command::new("reg")
                .args(["delete", RUN_REG_KEY, "/v", APP_VALUE_NAME, "/f"])
                .status();
            Ok(())
        }
    }

    /// Check whether SonyBridge is currently registered in HKCU Run.
    pub fn is_enabled() -> bool {
        Command::new("reg")
            .args(["query", RUN_REG_KEY, "/v", APP_VALUE_NAME])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}
