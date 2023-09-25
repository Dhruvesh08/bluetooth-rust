use clap::{Parser, Subcommand};

mod bluetooth;
pub use bluetooth::BluetoothSDK;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan for Bluetooth devices.
    Scan,
}

async fn main() -> Result<(), bluer::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan => {
            let sdk = BluetoothSDK::new().await?;
            sdk.scan_bluetooth().await?;
        }
    }

    Ok(())
}