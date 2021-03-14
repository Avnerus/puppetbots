extern crate argparse;
extern crate serde;
extern crate serde_json;
extern crate ws;
extern crate byteorder;
extern crate chrono;


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
use chrono::Local;

mod soft_error;
mod breakout_state;
mod ws_server;
mod game;

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
    let mut log = File::create("log.txt").unwrap();

    let (sensing_in, sensing_out) = channel();
    let (motor_in, motor_out): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

    let mut port_name = "".to_string();
    let baud_rate: u32 = 9600;

    println!("Starting server");

    // Server thread
    let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
        ws_server::start(
            Arc::clone(&config),
            sensing_out,
            motor_in
        );
    }).unwrap();
    
    let _ = server.join();
}
