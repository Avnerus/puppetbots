extern crate argparse;
extern crate serde;
extern crate serde_json;
extern crate ws;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc};
use std::sync::mpsc;

mod ws_server;
mod soft_error;
mod puppet;

#[derive(Deserialize, Serialize)]
struct ServerConfig {
    port: u16
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActuatorConfig {
    name: String,
    pressure_device: String,
    interface_type: String,
    contract_motor: u16,
    expand_motor: u16,
    speed_factor: f32
}


#[derive(Deserialize, Serialize)]
pub struct Config {
    server: ServerConfig,
    version: String,
    actuators: Vec<ActuatorConfig>
}

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new("./config.json");
    let file = File::open(path)?;
    let data = serde_json::from_reader(file)?;

    Ok(data)
}

fn main() {
    println!("Hello, Rusty WS server!");

    let config = Arc::new(read_config().unwrap());

    // Puppet thread
    let (puppet_tx, puppet_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();
    let (server_tx, server_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

    println!("Starting puppet");

    {
        let config = Arc::clone(&config);
        thread::Builder::new().name("puppet".to_owned()).spawn(move || {
            puppet::start(
                Arc::clone(&config),
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
