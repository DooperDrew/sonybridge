use clap::{Parser, Subcommand};
use sonybridge_core::protocol::MdrFrame;
use sonybridge_core::state::StateMachine;
use sonybridge_core::transport::MockTransport;
use sonybridge_core::transport::Transport;
use sonybridge_windows::{AudioEndpointSanitizer, WindowsBluetoothManager};

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
    /// Run invariant verification against the core MDR protocol parser
    VerifyCore,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Probe => {
            println!("=== SonyBridge M0 Diagnostic Probe ===");
            println!("\n[1/2] Scanning for paired Sony Bluetooth devices...");
            match WindowsBluetoothManager::discover_paired_sony_devices().await {
                Ok(devices) => {
                    if devices.is_empty() {
                        println!("  -> No paired Sony headphones detected.");
                        println!("  -> Tip: Ensure your WH-1000XM3 is paired in Windows Settings.");
                    } else {
                        println!("  -> Found {} paired device(s):", devices.len());
                        for d in &devices {
                            println!(
                                "     * Name: {}\n       MAC: {:012X}\n       Connected: {}",
                                d.name, d.mac_address, d.is_connected
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("  -> Bluetooth scan error: {e}");
                }
            }

            println!("\n[2/2] Inspecting Windows Core Audio endpoints...");
            match AudioEndpointSanitizer::enumerate_render_endpoints() {
                Ok(endpoints) => {
                    println!("  -> Found {} active render endpoint(s):", endpoints.len());
                    for ep in &endpoints {
                        let tag = if ep.is_hands_free {
                            "[Hands-Free AG - Degraded Quality]"
                        } else {
                            "[Stereo / Normal]"
                        };
                        println!("     * {tag} {}", ep.id);
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
                    "Name: {:30} MAC: {:012X} Connected: {}",
                    d.name, d.mac_address, d.is_connected
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
                println!("[{:10}] {}", status, ep.id);
            }
        }
        Commands::VerifyCore => {
            println!("Verifying core protocol state machine with mock transport...");
            let mut sm = StateMachine::new();
            let mut mock = MockTransport::new();

            // Set ANC
            let frame = sm.set_anc(sonybridge_core::models::AncMode::NoiseCanceling);
            mock.send(&frame.encode()).await?;
            println!("Sent ANC set frame: {:02X?}", frame.encode());

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
