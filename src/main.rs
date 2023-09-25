use bluer::{Adapter, BluetoothEvent};

async fn scan_devices(adapter: &Adapter) -> Result<(), Box<dyn std::error::Error>> {
    // Start discovery on the adapter
    adapter.start_discovery().await?;

    // Listen for Bluetooth events
    let mut events = adapter.events().await?;
    
    while let Some(event) = events.recv().await {
        match event {
            // When a new device is discovered
            BluetoothEvent::Device { address, name, .. } => {
                println!("Discovered device: {} ({})", name.unwrap_or_default(), address);
            }
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the default Bluetooth adapter
    let adapter = Adapter::default().await?;

    // Scan for devices
    scan_devices(&adapter).await?;

    Ok(())
}
