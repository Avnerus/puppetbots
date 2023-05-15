use std::sync::{Arc,Mutex};
use std::{thread};
use ws::{listen, CloseCode, Message, Sender, Handler, Handshake};
use ws::util::Token;
use ws;

use std::str;
use std::collections::HashMap;
use std::sync::mpsc;

use serde_json::json;

use crate::soft_error::SoftError;

use crate::Config;

const ADMIN_ROLE : u8 = 0;
const PUPPET_ROLE : u8 = 1;


#[derive(Serialize, Deserialize)]
struct Puppet {
    connected: bool,
    position: Vec<u8>,
    name: String,
    action: bool
}

struct ServerState {
    tokens: HashMap<Token, u8>,
    puppeteers: HashMap<u8, Sender>,
    pup_state: HashMap<u8, Puppet>,
    ws: Option<Sender>
}


// WebSocket connection handler for the server connection
struct Server {
   ws: Sender,
   state: Arc<Mutex<ServerState>>,
   server_tx: mpsc::Sender<Vec<u8>>,
   config: Arc<Config>
}

/*
fn send_pup_state(state: &ServerState, sender: &Sender) {
    println!("Sending puppet state");
    let json_command = json!({
        "command": "puppet-state",
        "state": state.pup_state
    });

    sender.broadcast((String::from("U") + &json_command.to_string()).as_bytes()).unwrap();
}*/

fn handle_message(
    server: &mut Server,
    msg: Message
) -> Result<(), SoftError> {
   // println!("Server got message '{}'. ", msg);
    let data = msg.into_data();
    let state = &mut *server.state.lock().unwrap();
    let command = data[0] as char;
    //println!("Command code: {}.", command);

    // Only command possible without a role is R-Register
    if command == 'R' {
        let role = data[1];
        println!("Register command role {}", role);
        match role {
            0 ..= 2 => {
                {
                    if let Some(_soft_target) = state.puppeteers.get(&role) {
                        return Err(SoftError::new("There is already a controller connected. Please try again later!"));
                    } else {
                         state.tokens.insert(server.ws.token(), role);
                         state.puppeteers.insert(
                             role,
                             server.ws.clone()
                         );                        
                         println!("Registration successful");
                    }
                }
            }

            _ => return Err(SoftError::new("Unknown role"))
        }
    }
    else {
        if let Some(_role) = state.tokens.get(&server.ws.token()) {
            match command {               
                'A' => {
                    // Actuator command
                    server.server_tx.send(data).unwrap();
                },
                'O' => {
                    // Orientation command
                    server.server_tx.send(data).unwrap();
                }
                'C' => {
                    // config command
                    let action = str::from_utf8(&data[1..4]).unwrap();
                    println!("config command {:?}", action);
                    match action {
                        "GET" => {
                            server.ws.send((String::from("F") + &json!(&server.config).to_string()).as_bytes()).unwrap();
                        },
                        "SET" => {
                            let config_json = str::from_utf8(&data[4..]).unwrap();   
                            let new_config:Config = serde_json::from_str(&config_json).unwrap();
                            server.config = Arc::new(new_config);
                            
                            server.server_tx.send(data).unwrap();
                        }                       
                        _ => return Err(SoftError::new("Unknown CONFIG command"))
                    }
                }
                _ => return Err(SoftError::new("Unknown command"))
            }
        } else {
            return Err(SoftError::new("Disconnected. Please refresh and try again."))
        }
    }

    Ok(())
}


impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        println!("Client connected!");
        let token = self.ws.token();
        println!("Client token: {:?}", token);        
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        match handle_message(
            self,
            msg
        ) {
            Err(err) => {
                println!("Error! {:?}", err);
                let mut prefix = "E".to_string();
                prefix.push_str(&err.message);
                self.ws.send(prefix).unwrap();
            },
            Ok(()) => {}
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Client disconnected! ({:?}, {})",code,reason);
        let state = &mut *self.state.lock().unwrap();

        if let Some(role) = state.tokens.get(&self.ws.token()) {
            println!("Disconnected from role {:}", role);
            state.puppeteers.remove(role);
            state.pup_state.remove(role);
        }
        state.tokens.remove(&self.ws.token()); 
    }
}

pub fn start(
    config: Arc<Config>,
    server_tx: mpsc::Sender<Vec<u8>>,
    puppet_rx: mpsc::Receiver<Vec<u8>>

) {
    println!("\nSpawning server on port {}", config.server.port);

    let state = Arc::new(Mutex::new(ServerState {
        tokens: HashMap::new(),
        puppeteers: HashMap::new(),
        pup_state : HashMap::new(),
        ws: None
    }));

    let sensing_state = state.clone();

    thread::spawn(move || {
        while let Ok(msg) = puppet_rx.recv() {
            let command = msg[0] as char;
            match command {
                'S' => {
                    let state = & sensing_state.lock().unwrap();
                    match msg[1] as char {
                        'P' => {
                        //    println!("Pressure sensing message!");
                            if let Some(sa) = state.puppeteers.get(&PUPPET_ROLE) {
                               sa.send(msg).unwrap();
                            }
                        }
                        _ => {
                            println!("Unknown sensing message! {}", msg[1]);
                        }
                    }
                }
                'D' => {
                    let state = & sensing_state.lock().unwrap();
                    println!("Debug message!");
                    if let Some(sa) = state.puppeteers.get(&ADMIN_ROLE) {
                       sa.send(msg).unwrap();
                    }
                }
                _ => {
                    println!("Unknown sensing command! {}",command);
                }
            }
        }
    });


    listen(("0.0.0.0",config.server.port), move |out| {
        println!("Connection");
        let server = Server {
            ws: out,
            state: Arc::clone(&state),
            server_tx: server_tx.clone(),
            config: Arc::clone(&config)
        };
        let state_mod = &mut state.lock().unwrap();
        state_mod.ws = Some(server.ws.clone());
        server
    }).unwrap();
}
