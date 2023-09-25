// Import required dependencies
use bluer::{Adapter, Address, DiscoveryFilter, DiscoveryTransport};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use std::collections::HashSet;

pub struct BluetoothSDK {
    adapter: Adapter,
}

impl BluetoothSDK {
    pub async fn new() -> Result<Self, bluer::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        Ok(Self { adapter })
    }

    pub async fn scan_bluetooth(
        &self,
        _with_changes: bool,
        _all_properties: bool,
        _le_only: bool,
        _br_edr_only: bool,
        _filter_addr: HashSet<Address>,
    ) -> Result<(), bluer::Error> {
        // Configure Bluetooth adapter and start scanning logic
        self.adapter.set_powered(true).await?;
    
        let filter = DiscoveryFilter {
            transport: DiscoveryTransport::Auto,
            ..Default::default()
        };
        self.adapter.set_discovery_filter(filter).await?;
    
        let device_events = self.adapter.discover_devices().await?;
        pin_mut!(device_events);
    
        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        // Handle device events
                        AdapterEvent::DeviceAdded(addr) => {
                            let res = self.query_device_details(addr).await;
                            if let Err(err) = res {
                                eprintln!("Error: {}", err);
                            }
                        }
                        // Handle device removal if needed
                        AdapterEvent::DeviceRemoved(_) => {
                            // You can add handling logic here if desired
                        }
                        _ => (),
                    }
                }
                else => break,
            }
        }
    
        Ok(())
    }


}


async fn query_device_details(&self, addr: Address) -> Result<(), bluer::Error> {
    let device = self.adapter.device(addr)?;

    println!("Device Address: {:?}", addr);
    println!("    Address type:       {:?}", device.address_type().await?);
    println!("    Name:               {:?}", device.name().await?);
    println!("    Icon:               {:?}", device.icon().await?);
    println!("    Class:              {:?}", device.class().await?);
    println!("    UUIDs:              {:?}", device.uuids().await?.unwrap_or_default());
    println!("    Paired:             {:?}", device.is_paired().await?);
    println!("    Connected:          {:?}", device.is_connected().await?);
    println!("    Trusted:            {:?}", device.is_trusted().await?);
    println!("    Modalias:           {:?}", device.modalias().await?);
    println!("    RSSI:               {:?}", device.rssi().await?);
    println!("    TX power:           {:?}", device.tx_power().await?);
    println!("    Manufacturer data:  {:?}", device.manufacturer_data().await?);
    println!("    Service data:       {:?}", device.service_data().await?);
    println!();

    Ok(())
}