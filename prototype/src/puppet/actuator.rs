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

#[derive(Copy, Clone, PartialEq)]
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
    pub set_state: State,
    pub speed: f32,
    pub delay: u16
}

impl ActuatorMessage {
    pub fn set_state(state:State, speed: f32, delay: u16) -> ActuatorMessage {
        ActuatorMessage {
            set_state: state,
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
    pub flow_speed: Arc<Mutex<f32>>,
    pub flow_change_per_sec: f32,
    pub flow_stop_angle: f32,
    pub max_pressure: i16,
    pub interface: Arc<Mutex<Box<dyn ActuatorInterface + Send + Sync>>>,
    rx: mpsc::Receiver<ActuatorMessage>,
    tx: mpsc::Sender<Vec<u8>>,
    action_queue: LinkedList<Box<ActuatorAction>>

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
            flow_speed: Arc::new(Mutex::new(0.0)),
            rx: props.rx,
            tx: props.tx,
            pressure: 0,
            max_pressure: props.max_pressure,
            flow_stop_angle: props.flow_stop_angle,
            flow_change_per_sec: props.flow_change_per_sec,
            state: Arc::new(Mutex::new(State::Idle)),
            action_queue: LinkedList::new()
        }
    } 
    pub fn start(&mut self) {  
        println!("Starting actuator {:?}",self.name);  

        let mut last_admin_update = Instant::now();  
        self.stop();
       
        loop {
            if let Ok(msg) = self.rx.try_recv() {
                /* The contracting state supports setting the flow speed before starting contraction.
                    Other states perform immediately */
                if msg.set_state == State::Contracting &&
                   (msg.speed - *self.flow_speed.lock().unwrap()).abs() >= MINIMUM_FLOW_DIFFERENCE {
                    let flow_change_time = (*self.flow_speed.lock().unwrap() - msg.speed).abs() / self.flow_change_per_sec * 1000.0;
                    let flow_action = if msg.speed > *self.flow_speed.lock().unwrap() {State::FlowIncreasing} else {State::FlowDecreasing};

                    println!(
                        "Should wait {:?}ms to go from speed {:?} to {:?}",
                        flow_change_time,
                        *self.flow_speed.lock().unwrap(),
                        msg.speed
                    );                          
                    self.action_queue.push_back(
                        Box::new(ActuatorAction {
                            state: flow_action,
                            speed: msg.speed,
                            delay: flow_change_time as u16                                    
                        })
                    );                   
                }                     
                self.action_queue.push_back(
                    Box::new(ActuatorAction {
                        state: msg.set_state,
                        speed: msg.speed,
                        delay: msg.delay
                    })
                )                                                       
            }
             /* else if let Some(config) = msg.set_config {
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
            // Only process actions when idle
            if *self.state.lock().unwrap() == State::Idle {
                if let Some(action) = self.action_queue.pop_front() {
                    println!("Performing action!");  
                    *self.state.lock().unwrap() = action.state;                                 
                    let mut target_flow_speed = *self.flow_speed.lock().unwrap();                  


                    match action.state {
                        State::FlowIncreasing => {
                            target_flow_speed = action.speed;
                            let mut int = self.interface.lock().unwrap();
                            int.set_flow_angle(ANTI_CLOCKWISE_MAX_ANGLE);
                        }
                        State::FlowDecreasing => {
                            target_flow_speed = action.speed;
                            let mut int = self.interface.lock().unwrap();

                            int.set_flow_angle(CLOCKWISE_MAX_ANGLE);
                        }
                        State::Contracting => {        
                            let mut int = self.interface.lock().unwrap();  
                            int.set_inlet_valve(1.0);
                            int.set_outlet_valve(0.0);                  
                                          
                        },
                        State::Expanding => {
                            let mut int = self.interface.lock().unwrap();

                            int.set_inlet_valve(0.0);
                            int.set_outlet_valve(1.0); 
                        },
                        State::Idle => {
                            self.stop();
                        },
                        State::FlowReset => {
                            self.reset_flow();
                        },                   
                    }
                    if action.delay > 0 {
                        // TODO: Redundant
                        let state = Arc::clone(&self.state);
                        let delay = action.delay;
                        let interface = Arc::clone(&self.interface);
                        let stop_angle = self.flow_stop_angle;
                        let flow_speed = Arc::clone(&self.flow_speed);

                        thread::spawn(move || {                           
                            println!("Waiting {}ms", delay);          
                            thread::sleep(Duration::from_millis(delay.into()));                            
                            println!("Done waiting");                          
                            let mut int = interface.lock().unwrap();
                            int.set_inlet_valve(0.0);
                            int.set_outlet_valve(0.0);
                            int.set_flow_angle(stop_angle);
                            *flow_speed.lock().unwrap() = target_flow_speed;
                            *state.lock().unwrap() = State::Idle;    
                        });                         
                    } else {
                        self.stop()                            
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
    fn stop(&mut self) {
        let mut int = self.interface.lock().unwrap();
        int.set_inlet_valve(0.0);
        int.set_outlet_valve(0.0);
        int.set_flow_angle(self.flow_stop_angle);
        *self.state.lock().unwrap() = State::Idle;
    }
    fn update(&mut self) {
        self.pressure = self.read_pressure();
        if self.pressure > self.max_pressure && *self.state.lock().unwrap() == State::Contracting {
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

        let flow_change_time = *self.flow_speed.lock().unwrap() / self.flow_change_per_sec * 1000.0;

        self.action_queue.push_back(
            Box::new(ActuatorAction {
                state: State::FlowDecreasing,
                speed: 0.0,
                delay: flow_change_time as u16                                   
            })
        );        
    }
}
