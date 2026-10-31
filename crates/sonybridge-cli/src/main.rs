use clap::{Parser, Subcommand};
use sonybridge_core::protocol::MdrFrame;
use sonybridge_core::state::StateMachine;
use sonybridge_core::transport::{MockTransport, Transport};
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
            sonybridge_windows::run_diagnostic_probe().await;
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
