extern crate argparse;
extern crate serde;
extern crate serde_json;
extern crate ws;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc,Mutex};
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};

use argparse::{ArgumentParser, Store};

mod ws_server;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    port: u16
}

#[derive(Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
    version: String
}

fn read_config() -> Result<Config, Box<std::error::Error>> {
    let path = Path::new("./config.json");
    let file = File::open(path)?;
    let data = serde_json::from_reader(file)?;

    Ok(data)
}

fn main() {
    println!("Hello, Rusty WS server!");

    let config = Arc::new(read_config().unwrap());

    let mut port_name = "".to_string();

    println!("Starting server");

    // Server thread
    let config_ws = Arc::clone(&config);
    let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
        ws_server::start(
            config_ws
        );
    }).unwrap();
    
    let _ = server.join();
}
