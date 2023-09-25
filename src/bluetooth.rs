use bluer::{Adapter, AdapterEvent, Address, DeviceEvent, DiscoveryFilter, DiscoveryTransport};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use std::{collections::HashSet, env};
use std::error::Error;

pub struct BluetoothSDK {
    
}

async fn query_device(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    println!("    Address type:       {}", device.address_type().await?);
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
    Ok(())
}

async fn query_all_device_properties(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    let props = device.all_properties().await?;
    for prop in props {
        println!("    {:?}", &prop);
    }
    Ok(())
}

impl BluetoothSDK {
    pub async fn new() -> Result<Self, Box<dyn Error>> {        
        Ok(Self )
    }

    // pub async fn turn_on_bluetooth(&self) -> Result<(), Box<dyn Error>> {
    //     self.adapter.set_powered(true).await?;
    //     Ok(())
    // }

    // pub async fn turn_off_bluetooth(&self) -> Result<(), Box<dyn Error>> {
    //     self.adapter.set_powered(false).await?;
    //     Ok(())
    // }

    pub async fn scan_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        let with_changes = env::args().any(|arg| arg == "--changes");
        let all_properties = env::args().any(|arg| arg == "--all-properties");
        let le_only = env::args().any(|arg| arg == "--le");
        let br_edr_only = env::args().any(|arg| arg == "--bredr");
        let filter_addr: HashSet<_> = env::args().filter_map(|arg| arg.parse::<Address>().ok()).collect();
    
        env_logger::init();
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        println!("Discovering devices using Bluetooth adapter {}\n", adapter.name());
        adapter.set_powered(true).await?;
    
        let filter = DiscoveryFilter {
            transport: if le_only {
                DiscoveryTransport::Le
            } else if br_edr_only {
                DiscoveryTransport::BrEdr
            } else {
                DiscoveryTransport::Auto
            },
            ..Default::default()
        };
        adapter.set_discovery_filter(filter).await?;
        println!("Using discovery filter:\n{:#?}\n\n", adapter.discovery_filter().await);
    
        let device_events = adapter.discover_devices().await?;
        pin_mut!(device_events);
    
        let mut all_change_events = SelectAll::new();
    
        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                                continue;
                            }
    
                            println!("Device added: {addr}");
                            let res = if all_properties {
                                query_all_device_properties(&adapter, addr).await
                            } else {
                                query_device(&adapter, addr).await
                            };
                            if let Err(err) = res {
                                println!("    Error: {}", &err);
                            }
    
                            if with_changes {
                                let device = adapter.device(addr)?;
                                let change_events = device.events().await?.map(move |evt| (addr, evt));
                                all_change_events.push(change_events);
                            }
                        }
                        AdapterEvent::DeviceRemoved(addr) => {
                            println!("Device removed: {addr}");
                        }
                        _ => (),
                    }
                    println!();
                }
                Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                    println!("Device changed: {addr}");
                    println!("    {property:?}");
                }
                else => break
            }
        }
    
        Ok(())
    }

    // pub async fn connect_bluetooth(&self, device_address: &str) -> Result<(), Box<dyn Error>> {
    //     self.adapter.start_discovery().await?;

    //     let mut events = self.adapter.events().await?;
    //     while let Some(event) = events.recv().await {
    //         match event {
    //             BluetoothEvent::Device { address, .. } if address == device_address => {
    //                 self.adapter.stop_discovery().await?;

    //                 let device = Device::new(self.adapter.session(), &address).await?;
    //                 device.connect().await?;

    //                 println!("Connected to device {}", device_address);
                    
    //                 return Ok(());
    //             }
    //             _ => {}
    //         }
    //     }

    //     Err("Device not found".into())
    // }
}
