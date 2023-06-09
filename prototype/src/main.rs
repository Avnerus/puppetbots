extern crate argparse;
extern crate serde;
extern crate serde_json;
extern crate ws;

#[macro_use]
extern crate serde_derive;

use std::thread;
use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc};
use std::sync::mpsc;

mod ws_server;
mod soft_error;
mod puppet;
mod util;

#[derive(Deserialize, Serialize, Debug)]
struct ServerConfig {
    port: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActuatorConfig {
    name: String,
    pressure_device_index: u16,
    max_pressure: i16,
    flow_change_time_ms: u16,
    flow_max_angle: u16,
    inlet_motor: u16,
    outlet_motor: u16,
    flow_control_servo: u16    
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    server: ServerConfig,
    version: String,
    interface_type: String,
    orientation_servo: u16,   
    actuators: Vec<ActuatorConfig>
}

fn read_config(config_file:String) -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new(&config_file);
    let file = File::open(path)?;
    let data = serde_json::from_reader(file)?;

    Ok(data)
}

fn main() {
    println!("Hello, Rusty WS server!");
    
    let args: Vec<String> = env::args().collect();
    let config_file = match args.len() {
        1 => "config.json",
        2 => &args[1],
        _ => panic!("Invalid number of command line arguments. Usage: {} [config file name]",  &args[0])
    };
    
    let config = Arc::new(read_config(config_file.into()).unwrap());

    // Puppet thread
    let (puppet_tx, puppet_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();
    let (server_tx, server_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

    println!("Starting puppet");

    {
        let config = Arc::clone(&config);
        thread::Builder::new().name("puppet".to_owned()).spawn(move || {
            puppet::start(
                config,
                puppet_tx,
                server_rx
            );
        }).unwrap();
    }


    // Server thread
    println!("Starting server");
    {
        let config = Arc::clone(&config);
        let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
            ws_server::start(
                config,
                server_tx,
                puppet_rx
            );
        }).unwrap();

        let _ = server.join();
    }
}
