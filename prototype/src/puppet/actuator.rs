#[cfg(not(target_os = "windows"))]
pub mod rpi_interface;

pub mod dummy_interface;

use std::sync::{mpsc, Arc, Mutex};
use std::{thread};
use std::time::{Duration, Instant};
use std::collections::LinkedList;

const ANTI_CLOCKWISE_MAX_ANGLE:f32 = 0.0;
const CLOCKWISE_MAX_ANGLE:f32 = 180.0;
const MINIMUM_FLOW_DIFFERENCE:f32 = 0.05;

#[derive(PartialEq)]
pub enum State {
    Contracting,
    Expanding,
    FlowIncreasing,
    FlowDecreasing,
    FlowReset,
    Idle
}

#[derive(PartialEq)]
pub enum ConfigMessage {
    MaxPressure,
    FlowChangePerSec    
}

pub struct ActuatorFlow {
    pub state: FlowState,
    pub speed: f32
}

pub struct ActuatorProps {
    pub name: String,
    pub interface: Box<dyn ActuatorInterface + Send + Sync>,
    pub flow_change_per_sec: f32,
    pub flow_stop_angle: f32,
    pub max_pressure: i16,
    pub rx: mpsc::Receiver<ActuatorMessage>,
    pub tx: mpsc::Sender<Vec<u8>>
}

pub struct ActuatorMessage {
    pub set_state: Option<State>,
    pub speed: f32,
    pub delay: u16
}

impl ActuatorMessage {
    pub fn set_state(state:State, speed: f32, delay: u16) -> ActuatorMessage {
        ActuatorMessage {
            set_state: Some(state),
            speed,
            delay
        }
    }
}

struct ActuatorAction {
    pub state: State,
    pub speed: f32,
    pub delay: u16
}

pub struct Actuator {
    pub name: String,
    pub pressure: i16,
    pub state: Arc<Mutex<State>>,
    pub flow_change_per_sec: f32,
    pub flow_stop_angle: f32,
    pub max_pressure: i16,
    pub interface: Arc<Mutex<Box<dyn ActuatorInterface + Send + Sync>>>,
    rx: mpsc::Receiver<ActuatorMessage>,
    tx: mpsc::Sender<Vec<u8>>
}

pub trait ActuatorInterface {
    fn set_inlet_valve(&mut self, throttle:f32);
    fn set_outlet_valve(&mut self, throttle:f32);
    fn read_pressure(&mut self) -> i16;
    fn update(&mut self);
    fn set_flow_angle(&mut self, angle: f32);        
}

impl Actuator {
    pub fn new(props: ActuatorProps) -> Self {
        Actuator {
            name: props.name,
            interface: Arc::new(Mutex::new(props.interface)),
            flow_state: Arc::new(Mutex::new({
                ActuatorFlow {
                    state: FlowState::IDLE,
                    speed: 0.0
                }
            })),
            rx: props.rx,
            tx: props.tx,
            pressure: 0,
            max_pressure: props.max_pressure,
            flow_stop_angle: props.flow_stop_angle,
            flow_change_per_sec: props.flow_change_per_sec,
            state: Arc::new(Mutex::new(State::IDLE))
        }
    } 
    pub fn start(&mut self) {  
        println!("Starting actuator {:?}",self.name);  

        let mut action_queue:LinkedList<Box<ActuatorAction>> = LinkedList::new();

        let mut last_admin_update = Instant::now();  

        self.stop();
        self.interface.lock().unwrap().set_flow_angle(self.flow_stop_angle);
       
        loop {

            if let Ok(msg) = self.rx.try_recv() {
                if let Some(state) = msg.set_state {          
                    action_queue.push_back(
                        Box::new(ActuatorAction {
                            state,
                            speed: msg.speed,
                            delay: msg.delay
                        })
                    );                    
                } /* else if let Some(config) = msg.set_config {
                    match config {
                        ConfigMessage::MaxPressure => {
                            println!("{} setting max pressure to {}", self.name, msg.value);
                            self.max_pressure = msg.value as i16;
                        },
                        ConfigMessage::FlowChangePerSec => {
                            println!("{} setting flow change per sec to {}", self.name, msg.value);   
                            self.flow_change_per_sec = msg.value;                         
                        }
                    }
                }*/

            }
            // Only process actions after flow changes (Automatically unlocks after the 'if')
            if self.flow_state.lock().unwrap().state == FlowState::IDLE &&
               *self.state.lock().unwrap() == State::IDLE             {
              
                if let Some(action) = action_queue.pop_front() {
                    println!("Performing action!");                                       
                    let state = action.state;
                    match state {
                        State::CONTRACTING => {
                            self.contract_at(action.speed);
                        },
                        State::EXPANDING => {
                            self.expand_at(action.speed);
                        },
                        State::IDLE => {
                            self.stop();
                        },
                        State::ResetFlow => {
                            self.reset_flow();
                        },                   
                    }
                    if action.delay > 0 {
                        let state = Arc::clone(&self.state);
                        let delay = action.delay;

                        thread::spawn(move || {                           
                            println!("Waiting {}ms", delay);          
                            thread::sleep(Duration::from_millis(delay.into()));                            
                            println!("Done waiting");                          
                            *state.lock().unwrap() = State::IDLE;                            
                        });                         
                    } else {

                        *self.state.lock().unwrap() = State::IDLE;                            
                    }
                }                    
            }
            self.update();
            if last_admin_update.elapsed().as_secs() >= 1 {
                last_admin_update = Instant::now();
                //println!("Admin update {}",self.pressure);
                let mut message = format!("SP{}",self.name).as_bytes().to_vec();
                message.extend(vec![0]);
                message.extend(self.pressure.to_le_bytes().to_vec());
                self.tx.send(message).unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
    fn contract_at(
        &mut self,
        mut speed: f32
    ) {
        println!("Contracting at {}", speed);
        if speed == 0.0 {
            self.stop();
        } else {           
            *self.state.lock().unwrap() = State::CONTRACTING;            

            let flow_state  = Arc::clone(&self.flow_state);
            let interface = Arc::clone(&self.interface);
            let flow_change = self.flow_change_per_sec;
            let stop_angle = self.flow_stop_angle;

            thread::spawn(move || {
                let mut flow_change_time = 0.0;
                {
                    let mut state = flow_state.lock().unwrap();
                    if (speed - state.speed).abs() >= MINIMUM_FLOW_DIFFERENCE {
                        flow_change_time = (state.speed- speed).abs() / flow_change * 1000.0;
                        println!(
                            "Should wait {:?}ms to go from speed {:?} to {:?}",
                            flow_change_time,
                            state.speed,
                            speed
                        );

                        if speed > state.speed {
                            state.state = FlowState::INCREASING;
                            interface.lock().unwrap().set_flow_angle(ANTI_CLOCKWISE_MAX_ANGLE);
                        } else {
                            state.state = FlowState::DECREASING;
                            interface.lock().unwrap().set_flow_angle(CLOCKWISE_MAX_ANGLE);
                        }   
                    } else {
                        speed = state.speed
                    }
                }
                thread::sleep(Duration::from_millis(flow_change_time as u64));
                println!("Done waiting");                  

                let mut int = interface.lock().unwrap();
                int.set_flow_angle(stop_angle);
                int.set_inlet_valve(1.0);
                int.set_outlet_valve(0.0);

                flow_state.lock().unwrap().state = FlowState::IDLE;
                flow_state.lock().unwrap().speed = speed;

                println!("Unlocked flow signal");
            });               
        }     
    }
    fn expand_at(&mut self, speed: f32) {
        println!("Expanding at {}!",speed);
        if speed == 0.0 {
            self.stop();
        } else {
            let mut int = self.interface.lock().unwrap();
            *self.state.lock().unwrap() = State::EXPANDING;
            int.set_inlet_valve(0.0);
            int.set_outlet_valve(speed);
        }  
    }
    fn stop(&mut self) {
        println!("Stopping actuator");
        let mut int = self.interface.lock().unwrap();
        int.set_inlet_valve(0.0);
        int.set_outlet_valve(0.0);
        *self.state.lock().unwrap() = State::IDLE;
    }
    fn update(&mut self) {
        self.pressure = self.read_pressure();
        if self.pressure > self.max_pressure && *self.state.lock().unwrap() == State::CONTRACTING {
            println!("Pressure surpassed MAX: {}", self.pressure);
            self.stop();
        } 
    }
    fn read_pressure(&mut self) -> i16 {
        self.interface.lock().unwrap().read_pressure()
    }
    fn reset_flow(
        &mut self        
    ) {
        println!("Resetting flow");
          
        *self.state.lock().unwrap() = State::IDLE;            

        let flow_state  = Arc::clone(&self.flow_state);
        let interface = Arc::clone(&self.interface);
        let flow_change = self.flow_change_per_sec;
        let stop_angle = self.flow_stop_angle;

        // TODO: Redundant
        thread::spawn(move || {
             
            let flow_change_time = flow_state.lock().unwrap().speed / flow_change * 1000.0;
            println!(
                "Should wait {:?}ms to go from speed {:?} to 0",
                flow_change_time,
                flow_state.lock().unwrap().speed   
            );
            flow_state.lock().unwrap().state = FlowState::DECREASING;
            interface.lock().unwrap().set_flow_angle(CLOCKWISE_MAX_ANGLE);         
            
            thread::sleep(Duration::from_millis(flow_change_time as u64));
            println!("Done waiting");                  

            let mut int = interface.lock().unwrap();
            int.set_flow_angle(stop_angle);
            int.set_inlet_valve(1.0);
            int.set_outlet_valve(0.0);

            flow_state.lock().unwrap().state = FlowState::IDLE;
            flow_state.lock().unwrap().speed = 0.0;

            println!("Unlocked flow signal");
        });        
    }
}
