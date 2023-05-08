use std::sync::{mpsc, Arc, Mutex};
use std::{thread};
use std::time::{Duration, Instant};
use std::collections::LinkedList;

use crate::puppet::hardware::{HardwareInterface};

const MINIMUM_FLOW_DIFFERENCE:f32 = 0.05;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Contracting,
    Expanding,
    FlowChange,
    Idle
}

#[derive(PartialEq)]
pub enum ConfigMessage {
    MaxPressure,
    FlowChangePerSec    
}

pub struct ActuatorProps {
    pub name: String,
    pub interface: Arc<dyn HardwareInterface + Send + Sync>,
    pub flow_change_time_ms: u16,
    pub flow_max_angle: u16,
    pub max_pressure: i16,
    pub pressure_device_index: u16,
    pub inlet_motor: u16,
    pub outlet_motor: u16,
    pub flow_control_servo: u16,
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
    pub flow_change_time_ms: u16,
    pub flow_max_angle: u16,
    pub max_pressure: i16,
    pub pressure_device_index: u16,
    pub inlet_motor: u16,
    pub outlet_motor: u16,
    pub flow_control_servo: u16,
    pub interface: Arc<Mutex<Box<dyn ActuatorInterface + Send + Sync>>>,
    rx: mpsc::Receiver<ActuatorMessage>,
    tx: mpsc::Sender<Vec<u8>>,
    action_queue: LinkedList<Box<ActuatorAction>>

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
            flow_change_time_ms: props.flow_change_time_ms,
            flow_max_angle: props.flow_max_angle,
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

                    println!(
                        "Should wait {:?}ms to go from speed {:?} to {:?}",
                        self.flow_change_time_ms,
                        *self.flow_speed.lock().unwrap(),
                        msg.speed
                    );                          
                    self.action_queue.push_back(
                        Box::new(ActuatorAction {
                            state: State::FlowChange,
                            speed: msg.speed,
                            delay: self.flow_change_time_ms                                  
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
                        State::FlowChange => {
                            target_flow_speed = action.speed;
                            let flow_angle = self.flow_max_angle as f32 * target_flow_speed;
                            let mut int = self.interface.lock().unwrap();
                            int.set_servo_angle(self.flow_control_servo, flow_angle);
                        }            
                        State::Contracting => {        
                            let mut int = self.interface.lock().unwrap();  
                            int.set_dc_motor(self.inlet_motor, 1.0);
                            int.set_dc_motor(self.outlet_motor, 0.0);                                          
                        },
                        State::Expanding => {
                            let mut int = self.interface.lock().unwrap();
                            int.set_dc_motor(self.inlet_motor, 0.0);
                            int.set_dc_motor(self.outlet_motor, 1.0);                            
                        },
                        State::Idle => {
                            self.stop();
                        }                        
                    }
                    if action.delay > 0 {
                        // TODO: Redundant
                        let state = Arc::clone(&self.state);
                        let delay = action.delay;
                        let interface = Arc::clone(&self.interface);
                        let flow_speed = Arc::clone(&self.flow_speed);

                        thread::spawn(move || {                           
                            println!("Waiting {}ms", delay);          
                            thread::sleep(Duration::from_millis(delay.into()));                            
                            println!("Done waiting");                          
                            let mut int = interface.lock().unwrap();
                            int.set_inlet_valve(0.0);
                            int.set_outlet_valve(0.0);
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
        self.set_dc_motor(self.inlet_motor, 0.0);
        self.set_dc_motor(self.outlet_motor, 0.0);
        
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
        self.interface.lock().unwrap().read_adc(self.pressure_device_index);
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
