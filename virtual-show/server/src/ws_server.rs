use std::sync::{Arc,Mutex};
use ws::{listen, CloseCode, Message, Sender, Handler, Handshake};
use ws::util::Token;
use ws;

use std::thread::{JoinHandle};
use std::thread;
use std::sync::mpsc;
use std::sync::MutexGuard;
use std::str;
use std::rc::Rc;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use serde::{Deserialize, Serialize};
use serde_json::json;

use soft_error::SoftError;
use ytchat::YTChatMessage;

use Config;

const  WAITING:u8 =  0;
const  READY:u8 = 1;
const  CHOSE_1:u8 = 2;
const  CHOSE_2:u8 = 3;
const  EXPLAIN_1:u8 = 4;
const  EXPLAIN_2:u8 = 5;
const  DONE_1:u8 = 6;
const  DONE_2:u8 = 7;

const LAST_EXPLAINED:usize = 2;

const CONTROLLER_TIMEOUT:Duration = Duration::from_secs(600);
const TYPING_TIMEOUT:Duration = Duration::from_secs(2);

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
  //  serial: ThreadOut<String>,
   config: Arc<Config>,
   state: Arc<Mutex<ServerState>>
}


fn send_pup_state(state: &ServerState, sender: &Sender) {
    println!("Sending puppet state");
    let json_command = json!({
        "command": "puppet-state",
        "state": state.pup_state
    });

    sender.broadcast((String::from("U") + &json_command.to_string()).as_bytes());
}

fn handle_message(
    server: &mut Server,
    msg: Message
) -> Result<(), SoftError> {
    println!("Server got message '{}'. ", msg);
    let data = msg.into_data();
    let mut state = &mut *server.state.lock().unwrap();
    let command = data[0] as char;
    println!("Command code: {}.", command);

    // Only command possible without a role is R-Register
    if command == 'R' {
        let role = data[1];
        println!("Register command role {}", role);
        match role {
            0 ..= 2 => {
                {
                    if let Some(soft_target) = state.puppeteers.get(&role) {
                        return Err(SoftError::new("There is already a controller connected. Please try again later!"));
                    } else {
                         state.tokens.insert(server.ws.token(), role);
                         state.puppeteers.insert(
                             role,
                             server.ws.clone()
                         );
                         state.pup_state.insert (
                             role,
                             Puppet {
                                connected: true,
                                position: match role {
                                    1 => vec![105,103],
                                    2 => vec![180,103],
                                    _ => vec![0,0]
                                },
                                name: str::from_utf8(&data[2..]).unwrap().to_string(),
                                action: false
                             }
                         );
                         println!("Registration successful");
                    }
                }
                send_pup_state(state, &server.ws);
            }

            _ => return Err(SoftError::new("Unknown role"))
        }
    }
    else {
        if let Some(role) = state.tokens.get(&server.ws.token()) {
            match command {
                'S' => {
                    // app state command
                    let app = str::from_utf8(&data[1..4]).unwrap();
                    println!("state command {:?}", app);
                    match app {
                        "POS" => {
                            let pos_state_x:u8 = data[4];
                            let pos_state_y:u8 = data[5];
                            println!("POS State! {:?}", [pos_state_x,pos_state_y]);
                            (*state.pup_state.get_mut(&role).unwrap()).position[0] = pos_state_x;
                            (*state.pup_state.get_mut(&role).unwrap()).position[1] = pos_state_y;
                            send_pup_state(state, &server.ws);
                        },
                        "ACT" => {
                            let act:bool = (data[4] == 1);
                            println!("ACT State! {:?}", act);
                            (*state.pup_state.get_mut(&role).unwrap()).action = act;
                            send_pup_state(state, &server.ws);
                        },
                        _ => return Err(SoftError::new("Unknown command"))
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
        let state = &*self.state.lock().unwrap();
        send_pup_state(state, &self.ws);
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
                self.ws.send(prefix);
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

        send_pup_state(state, &self.ws);
        state.tokens.remove(&self.ws.token()); 
    }
}

pub fn start(
    config: Arc<Config>,
    ytchat_rx: mpsc::Receiver<Vec<YTChatMessage>>

) {
    println!("\nSpawning server on port {}", config.server.port);

    let state = Arc::new(Mutex::new(ServerState {
        tokens: HashMap::new(),
        puppeteers: HashMap::new(),
        pup_state : HashMap::new(),
        ws: None
    }));

   let ytchat_state = state.clone();
   let ytchat_thread = thread::spawn(move || {
       while let Ok(mut msg) = ytchat_rx.recv() {
           println!("Message!");
           let mut state_mod = &mut ytchat_state.lock().unwrap();
           if let Some(ws) = &state_mod.ws {
                let json_command = json!({
                    "command": "youtube-chat",
                    "messages": msg
                });
                ws.broadcast((String::from("U") + &json_command.to_string()).as_bytes());
           }
       }
   });

    listen(("0.0.0.0",config.server.port), move |out| {
        println!("Connection");
        let server = Server {
            ws: out,
            config: Arc::clone(&config),
            state: Arc::clone(&state)
        };
        let mut state_mod = &mut state.lock().unwrap();
        state_mod.ws = Some(server.ws.clone());
        server
    }).unwrap();
}
