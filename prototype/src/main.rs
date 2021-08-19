extern crate argparse;
extern crate serde;
extern crate serde_json;
extern crate ws;
#[macro_use]
extern crate serde_derive;

extern crate ads1x1x;
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate nb;
extern crate adafruit_motorkit;


use std::thread;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc};
use std::sync::mpsc;

mod ws_server;
mod soft_error;
mod actuator;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    port: u16
}

#[derive(Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
    version: String
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

    // Actuator thread
    let (actuator_tx, actuator_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

    println!("Starting actuator");

    let actuator_thread = thread::Builder::new().name("actuator".to_owned()).spawn(move || {
        actuator::start(
            actuator_rx
        );
    }).unwrap();


    // Server thread
    let (server_tx, server_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

    println!("Starting server");
    let config_ws = Arc::clone(&config);
    let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
        ws_server::start(
            config_ws,
            server_tx
        );
    }).unwrap();
    
    let _ = server.join();
}
