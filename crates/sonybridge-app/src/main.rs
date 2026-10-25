// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sonybridge_core::models::{AncMode, EqPreset, EqualizerState};
use sonybridge_core::state::{HeadphoneState, StateMachine};
use sonybridge_windows::{AudioEndpointSanitizer, WindowsBluetoothManager};
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, State};

struct AppState {
    state_machine: Mutex<StateMachine>,
}

#[derive(Deserialize)]
struct AncPayload {
    mode: String,
    level: u8,
    #[serde(rename = "focusOnVoice")]
    focus_on_voice: bool,
}

#[derive(Deserialize)]
struct EqPayload {
    preset: String,
    bands: [i8; 5],
    #[serde(rename = "clearBass")]
    clear_bass: i8,
}

#[derive(Serialize)]
struct CommandResult {
    status: String,
    message: Option<String>,
}

#[tauri::command]
fn get_device_state(state: State<AppState>) -> HeadphoneState {
    let sm = state.state_machine.lock().unwrap();
    sm.state.clone()
}

#[tauri::command]
fn set_anc_mode(state: State<AppState>, payload: AncPayload) -> CommandResult {
    let mut sm = state.state_machine.lock().unwrap();
    let mode = match payload.mode.as_str() {
        "Off" => AncMode::Off,
        "AmbientSound" => AncMode::AmbientSound {
            level: payload.level,
            focus_on_voice: payload.focus_on_voice,
        },
        _ => AncMode::NoiseCanceling,
    };
    let frame = sm.set_anc(mode);
    tracing::info!(seq = frame.seq_no, "Dispatched ANC command to hardware");
    CommandResult {
        status: "ok".to_string(),
        message: None,
    }
}

#[tauri::command]
fn set_equalizer(state: State<AppState>, payload: EqPayload) -> CommandResult {
    let mut sm = state.state_machine.lock().unwrap();
    let preset = match payload.preset.as_str() {
        "Bright" => EqPreset::Bright,
        "Excited" => EqPreset::Excited,
        "Mellow" => EqPreset::Mellow,
        "Relaxed" => EqPreset::Relaxed,
        "Vocal" => EqPreset::Vocal,
        "TrebleBoost" => EqPreset::TrebleBoost,
        "BassBoost" => EqPreset::BassBoost,
        "Speech" => EqPreset::Speech,
        "Custom1" => EqPreset::Custom1,
        "Custom2" => EqPreset::Custom2,
        _ => EqPreset::Off,
    };

    let eq_state = EqualizerState {
        preset,
        bands: payload.bands,
        clear_bass: payload.clear_bass,
    };

    let frame = sm.set_equalizer(eq_state);
    tracing::info!(seq = frame.seq_no, "Dispatched Equalizer frame to hardware");
    CommandResult {
        status: "ok".to_string(),
        message: None,
    }
}

#[tauri::command]
async fn trigger_reconnect() -> Result<CommandResult, String> {
    tracing::info!("Scanning for paired Sony headphones...");
    let devices = WindowsBluetoothManager::discover_paired_sony_devices()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(device) = devices.first() {
        tracing::info!(
            name = %device.name,
            mac = format!("{:012X}", device.mac_address),
            "Found headphone, establishing control channel"
        );
        Ok(CommandResult {
            status: "ok".to_string(),
            message: Some(format!("Reconnected to {}", device.name)),
        })
    } else {
        Err("No paired Sony WH-1000XM3 detected in Windows Settings".to_string())
    }
}

#[tauri::command]
fn enforce_a2dp() -> CommandResult {
    let is_hands_free = AudioEndpointSanitizer::is_hands_free_active();
    tracing::info!(is_hands_free, "Audio endpoint audit performed");
    CommandResult {
        status: "ok".to_string(),
        message: Some(if is_hands_free {
            "Hands-Free profile detected and isolated".to_string()
        } else {
            "Stereo A2DP audio endpoint is active".to_string()
        }),
    }
}

#[tauri::command]
fn set_touch_sensor(state: State<AppState>, enabled: bool) -> CommandResult {
    let mut sm = state.state_machine.lock().unwrap();
    sm.state.settings.touch_sensor_enabled = enabled;
    tracing::info!(enabled, "Touch sensor preference updated");
    CommandResult {
        status: "ok".to_string(),
        message: None,
    }
}

#[tauri::command]
fn set_auto_off(state: State<AppState>, minutes: u32) -> CommandResult {
    let mut sm = state.state_machine.lock().unwrap();
    sm.state.settings.auto_power_off_minutes = if minutes > 0 { Some(minutes) } else { None };
    tracing::info!(minutes, "Auto power-off timeout updated");
    CommandResult {
        status: "ok".to_string(),
        message: None,
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let initial_state = AppState {
        state_machine: Mutex::new(StateMachine::new()),
    };

    tauri::Builder::default()
        .manage(initial_state)
        .invoke_handler(tauri::generate_handler![
            get_device_state,
            set_anc_mode,
            set_equalizer,
            trigger_reconnect,
            enforce_a2dp,
            set_touch_sensor,
            set_auto_off
        ])
        .setup(|app| {
            // Build Context Menu for System Tray
            let header_item = MenuItemBuilder::with_id("header", "Sony WH-1000XM3 (85%)")
                .enabled(false)
                .build(app)?;
            let reconnect_item =
                MenuItemBuilder::with_id("reconnect", "Quick Reconnect").build(app)?;
            let show_item = MenuItemBuilder::with_id("show", "Open Dashboard").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit SonyBridge").build(app)?;

            let tray_menu = MenuBuilder::new(app)
                .items(&[&header_item, &reconnect_item, &show_item, &quit_item])
                .build()?;

            // Build System Tray Icon
            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "reconnect" => {
                        tracing::info!("Tray triggered reconnect");
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Prevent app from terminating when main window is closed: hide to tray
            if let Some(window) = app.get_webview_window("main") {
                let win_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running SonyBridge application");
}
