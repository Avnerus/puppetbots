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
use ytchat::YTChatMessage;

mod soft_error;
mod ws_server;
mod ytchat;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    port: u16
}

#[derive(Deserialize, Debug)]
struct YTChatConfig {
    enabled: bool,
    chat_id: String,
    api_key: String
}

#[derive(Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
    ytchat: YTChatConfig,
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

    let (ytchat_tx, ytchat_rx): (Sender<Vec<YTChatMessage>>, Receiver<Vec<YTChatMessage>>) = channel();

    let mut port_name = "".to_string();
    let baud_rate: u32 = 9600;

    println!("Starting server");

    // Youtube Chat thread
    let config_yt = Arc::clone(&config);
    let ytchat_thread = thread::Builder::new().name("server".to_owned()).spawn(move || {
        ytchat::start(
            config_yt,
            ytchat_tx
        );
    }).unwrap();

    // Server thread
    let config_ws = Arc::clone(&config);
    let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
        ws_server::start(
            config_ws,                        
            ytchat_rx
        );
    }).unwrap();
    
    let _ = ytchat_thread.join();
    let _ = server.join();
}
