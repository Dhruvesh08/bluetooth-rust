use clap::{App, Arg, SubCommand};
use tokio::runtime::Runtime;

mod bluetooth;
use bluetooth::BluetoothSDK;
fn main() {
    let matches = App::new("Bluetooth CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Interacts with Bluetooth devices")
        .subcommand(SubCommand::with_name("on").about("Turns on Bluetooth"))
        .subcommand(SubCommand::with_name("off").about("Turns off Bluetooth"))
        .subcommand(SubCommand::with_name("scan").about("Scans for Bluetooth devices"))
        .subcommand(
            SubCommand::with_name("connect")
                .about("Connects to a Bluetooth device")
                .arg(Arg::with_name("ADDRESS").required(true).index(1)),
        )
        .get_matches();

    let mut rt = Runtime::new().unwrap();
    let sdk = rt.block_on(BluetoothSDK::new()).unwrap();

    match matches.subcommand() {
        // ("on", Some(_)) => {
        //     BluetoothSDK::turn_on_bluetooth(&sdk).unwrap();
        // }
        // ("off", Some(_)) => {
        //    BluetoothSDK::turn_off_bluetooth(&sdk).unwrap();
        // }
        // ("scan", Some(_)) => {
        //    BluetoothSDK::scan_bluetooth(&sdk).unwrap();
        // }
        ("connect", Some(connect_matches)) => {
            let address = connect_matches.value_of("ADDRESS").unwrap();
            rt.block_on(sdk.connect_bluetooth(address)).unwrap();
        }
        _ => {}
    }
}