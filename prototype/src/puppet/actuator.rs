#[cfg(not(target_os = "windows"))]
pub mod rpi_interface;

pub mod dummy_interface;

use std::sync::{mpsc, Arc, Mutex};
use std::{thread};
use std::time::{Duration, Instant};
use std::collections::LinkedList;

#[derive(PartialEq)]
pub enum State {
    CONTRACTING,
    EXPANDING,
    IDLE
}

#[derive(PartialEq)]
pub enum FlowState {
    INCREASING,
    DECREASING,
    IDLE
}

pub struct ActuatorProps {
    pub name: String,
    pub interface: Box<dyn ActuatorInterface + Send>,
    pub flow_change_per_sec: f32,
    pub max_pressure: i16,
    pub rx: mpsc::Receiver<ActuatorMessage>,
    pub tx: mpsc::Sender<Vec<u8>>
}

pub struct ActuatorMessage {
    pub set_state: State,
    pub speed: f32    
}

struct ActuatorAction {
    pub msg: ActuatorMessage,
    pub delay: Duration
}

pub struct Actuator {
    pub name: String,
    pub pressure: i16,
    pub state: State,
    pub flow_state: FlowState, 
    pub flow_change_per_sec: f32,
    pub max_pressure: i16,
    pub interface: Box<dyn ActuatorInterface + Send>,
    rx: mpsc::Receiver<ActuatorMessage>,
    tx: mpsc::Sender<Vec<u8>>
}

pub trait ActuatorInterface {
    fn set_inlet_valve(&mut self, throttle:f32);
    fn set_outlet_valve(&mut self, throttle:f32);
    fn read_pressure(&mut self) -> i16;
    fn update(&mut self);
    fn start_flow_increase(&mut self);
    fn start_flow_decrease(&mut self);
    fn maintain_current_flow(&mut self);            
}

impl Actuator {
    pub fn new(props: ActuatorProps) -> Self {
        Actuator {
            name: props.name,
            interface: props.interface,
            rx: props.rx,
            tx: props.tx,
            pressure: 0,
            max_pressure: props.max_pressure,
            flow_change_per_sec: props.flow_change_per_sec,
            state: State::IDLE,
            flow_state: FlowState::IDLE
        }
    } 
    pub fn start(&mut self) {  
        println!("Starting actuator {:?}",self.name);  

        let mut action_queue:LinkedList<Box<ActuatorAction>> = LinkedList::new();

        let mut last_admin_update = Instant::now();
        let mut last_message_instant:Option<Instant> = None;
        
        let flow_control_pause = Arc::new(Mutex::new(false)); 

        // 0 means that the servo valve starts as closed. 
        let current_flow_speed = Arc::new(Mutex::new(0.0));


        let mut waiting_action:Option<Box<ActuatorAction>> = None;
        let mut action_time = Instant::now();
       
        loop {

            if let Ok(msg) = self.rx.try_recv() {
                let delay = if self.state == State::IDLE {
                    Duration::ZERO
                } else {
                    match last_message_instant {
                        Some(instant) => Instant::now().duration_since(instant),
                        None => Duration::ZERO
                    }
                };       
        
                action_queue.push_back(
                    Box::new(ActuatorAction {
                        msg,
                        delay
                    })
                );
                last_message_instant = Some(Instant::now());
            }
            let paused = flow_control_pause.lock().unwrap();
            if !*paused {

                if Option::is_none(&waiting_action) {
                    if let Some(action) = action_queue.pop_front() {
                        let delay = action.delay;
                        waiting_action = Some(action);
                        println!("Delaying action for {:?}", delay);
                        action_time = Instant::now() + delay;
                    }       
                }
                else if Instant::now() >= action_time {    
                    println!("Performing action!");                     
                    let action = waiting_action.unwrap();
                    match action.msg.set_state {
                        State::CONTRACTING => {
                            self.contract_at(
                                action.msg.speed,
                                Arc::clone(&flow_control_pause),
                                Arc::clone(&current_flow_speed)
                            );
                        },
                        State::EXPANDING => {
                            self.expand_at(action.msg.speed);
                        },
                        State::IDLE => {
                            self.stop();
                        }                    
                    }
                    waiting_action = None;
                }    
            } 

            self.update();
            if last_admin_update.elapsed().as_secs() >= 1 {
                last_admin_update = Instant::now();
                // println!("Admin update {}",self.pressure);
                let mut message = format!("SP{}",self.name).as_bytes().to_vec();
                message.extend(vec![0]);
                message.extend(self.pressure.to_le_bytes().to_vec());
                self.tx.send(message).unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
    fn contract_at(&mut self, speed: f32, flow_signal:Arc<Mutex<bool>>, current_flow_speed:Arc<Mutex<f32>>) {
        println!("Contracting at {}", speed);
        if speed == 0.0 {
            self.stop();
        } else {           
            let flow_speed = current_flow_speed.lock().unwrap();
            if speed != *flow_speed {
                let flow_change_time = (*flow_speed - speed).abs() / self.flow_change_per_sec * 1000.0;
                println!(
                    "Should wait {:?}ms to go from speed {:?} to {:?}",
                    flow_change_time,
                    *flow_speed,
                    speed
                );
                if speed > *flow_speed {
                    self.flow_state = FlowState::INCREASING;
                    self.interface.start_flow_increase();                    
                } else {
                    self.flow_state = FlowState::DECREASING;
                    self.interface.start_flow_decrease();
                }   
                drop(flow_speed);
                let flow_speed_c = Arc::clone(&current_flow_speed);

                thread::spawn(move || {
                    { *(flow_signal.lock().unwrap()) = true; }                    
                    thread::sleep(Duration::from_millis(flow_change_time as u64));
                    { *(flow_speed_c.lock().unwrap()) = speed; }                
                    println!("Done waiting");                  
                    { *(flow_signal.lock().unwrap()) = false; }                    

                });               
            }
            self.state = State::CONTRACTING;            
            self.interface.set_inlet_valve(speed);
            self.interface.set_outlet_valve(0.0);
        }     
    }
    fn expand_at(&mut self, speed: f32) {
        println!("Expanding at {}!",speed);
        if speed == 0.0 {
            self.stop();
        } else {
            self.state = State::EXPANDING;
            self.interface.set_inlet_valve(0.0);
            self.interface.set_outlet_valve(speed);
        }  
    }
    fn stop(&mut self) {
        println!("Stopping actuator");
        self.interface.set_inlet_valve(0.0);
        self.interface.set_outlet_valve(0.0);
        self.state = State::IDLE;
    }
    fn update(&mut self) {
        self.pressure = self.read_pressure();
        if self.pressure > self.max_pressure && self.state == State::CONTRACTING {
            println!("Pressure surpassed MAX: {}", self.pressure);
            self.stop();
        } 
    }
    fn read_pressure(&mut self) -> i16 {
        self.interface.read_pressure()
    }
}