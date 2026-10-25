use clap::{Parser, Subcommand};
use sonybridge_core::commands;
use sonybridge_core::protocol::MdrFrame;
use sonybridge_core::state::StateMachine;
use sonybridge_core::transport::{MockTransport, Transport};
use sonybridge_windows::{AudioEndpointSanitizer, WinRtRfcommTransport, WindowsBluetoothManager};

#[derive(Parser, Debug)]
#[command(name = "sonybridge-cli")]
#[command(about = "SonyBridge Diagnostic Probe & Testing Utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Probe the local Windows environment for paired Sony headphones and audio endpoints (M0 Spike)
    Probe,
    /// List all paired Bluetooth devices matching Sony signatures
    Devices,
    /// Inspect active Windows audio render endpoints and check for Hands-Free degradation
    Audio,
    /// Enforce Stereo A2DP default audio endpoint routing
    EnforceA2dp,
    /// Run invariant verification against the core MDR protocol parser
    VerifyCore,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Probe => {
            println!("=== SonyBridge M0/M9 Diagnostic Probe ===");
            println!("\n[1/3] Scanning for paired Sony Bluetooth devices...");
            match WindowsBluetoothManager::discover_paired_sony_devices().await {
                Ok(devices) => {
                    if devices.is_empty() {
                        println!("  -> No paired Sony headphones detected.");
                        println!("  -> Tip: Ensure your WH-1000XM3 is paired in Windows Settings.");
                    } else {
                        println!("  -> Found {} paired device(s):", devices.len());
                        for d in &devices {
                            println!(
                                "     * Name: {}\n       MAC: {:012X}\n       Connected: {}\n       RFCOMM Services: {:?}",
                                d.name, d.mac_address, d.is_connected, d.rfcomm_services
                            );
                        }

                        if let Some(active) = devices.iter().find(|d| d.is_connected) {
                            println!(
                                "\n[2/3] Probing live RFCOMM MDR channel on {}...",
                                active.name
                            );
                            let mut transport =
                                WinRtRfcommTransport::with_mac(&active.name, active.mac_address);
                            match transport.connect().await {
                                Ok(()) => {
                                    println!("  -> RFCOMM StreamSocket connected!");
                                    let query =
                                        MdrFrame::new(0, commands::build_v1_battery_query());
                                    if transport.send(&query.encode_wire()).await.is_ok() {
                                        println!(
                                            "  -> Sent MDR v1 Battery Query: {:02X?}",
                                            query.encode_wire()
                                        );
                                    }
                                    let _ = transport.disconnect().await;
                                }
                                Err(e) => {
                                    println!("  -> RFCOMM connect note: {e}");
                                }
                            }
                        } else {
                            println!("\n[2/3] Skipping live RFCOMM handshake (headphones currently powered off / disconnected).");
                        }
                    }
                }
                Err(e) => {
                    println!("  -> Bluetooth scan error: {e}");
                }
            }

            println!("\n[3/3] Inspecting Windows Core Audio endpoints...");
            match AudioEndpointSanitizer::enumerate_render_endpoints() {
                Ok(endpoints) => {
                    println!("  -> Found {} active render endpoint(s):", endpoints.len());
                    for ep in &endpoints {
                        let tag = if ep.is_hands_free {
                            "[Hands-Free AG - Degraded Quality]"
                        } else {
                            "[Stereo / Normal]"
                        };
                        let def = if ep.is_default { " (DEFAULT)" } else { "" };
                        println!("     * {tag}{def} {}", ep.id);
                    }
                }
                Err(e) => {
                    println!("  -> Audio endpoint enumeration error: {e}");
                }
            }
            println!("\n=== Diagnostic Probe Completed ===");
        }
        Commands::Devices => {
            let devices = WindowsBluetoothManager::discover_paired_sony_devices().await?;
            println!("Discovered {} paired Sony device(s):", devices.len());
            for d in devices {
                println!(
                    "Name: {:30} MAC: {:012X} Connected: {} Services: {}",
                    d.name,
                    d.mac_address,
                    d.is_connected,
                    d.rfcomm_services.len()
                );
            }
        }
        Commands::Audio => {
            let endpoints = AudioEndpointSanitizer::enumerate_render_endpoints()?;
            println!("Active Audio Render Endpoints ({}):", endpoints.len());
            for ep in endpoints {
                let status = if ep.is_hands_free {
                    "HANDS-FREE"
                } else {
                    "STEREO"
                };
                let def = if ep.is_default { "*DEFAULT*" } else { "" };
                println!("[{:10}] {:9} {}", status, def, ep.id);
            }
        }
        Commands::EnforceA2dp => match AudioEndpointSanitizer::enforce_stereo_a2dp() {
            Ok(id) => println!("Successfully enforced Stereo A2DP default endpoint: {id}"),
            Err(e) => println!("Could not enforce Stereo A2DP endpoint: {e}"),
        },
        Commands::VerifyCore => {
            println!("Verifying core protocol state machine with mock transport...");
            let mut sm = StateMachine::new();
            let mut mock = MockTransport::new();

            // Set ANC
            let frame = sm.set_anc(sonybridge_core::models::AncMode::NoiseCanceling);
            mock.send(&frame.encode()).await?;
            println!("Sent ANC set frame: {:02X?}", frame.encode());

            // Set V1 Wire ANC
            let v1_frame = sm.set_v1_anc(sonybridge_core::models::AncMode::NoiseCanceling);
            mock.send(&v1_frame.encode_wire()).await?;
            println!("Sent V1 Wire ANC frame: {:02X?}", v1_frame.encode_wire());

            // Simulate incoming battery response from headphones
            let incoming_battery_frame = MdrFrame::new(1, vec![0x02, 0x01, 88, 0x00]);
            sm.handle_incoming_frame(&incoming_battery_frame)
                .map_err(std::io::Error::other)?;

            println!(
                "Core State updated successfully: Battery = {}%",
                sm.state.battery.percentage
            );
            assert_eq!(sm.state.battery.percentage, 88);
            println!("Core invariant verification: PASS");
        }
    }

    Ok(())
}
