#[cfg(not(target_os = "windows"))]
pub mod rpi_interface;

pub mod dummy_interface;

use std::sync::mpsc;
use std::{thread};
use std::time::{Duration, Instant};


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

pub struct Actuator {
    pub name: String,
    pub pressure: i16,
    pub state: State,
    pub flow_state: FlowState,
    pub current_flow_speed: f32,
    pub flow_change_per_sec: f32,
    pub max_pressure: i16,
    pub interface: Box<dyn ActuatorInterface + Send>,
    rx: mpsc::Receiver<ActuatorMessage>,
    tx: mpsc::Sender<Vec<u8>>
}

pub trait ActuatorInterface: Send {
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
            flow_state: FlowState::IDLE,
            // TODO: This means that the valve starts as closed. 
            current_flow_speed: 0.0
        }
    } 
    pub fn start(&mut self) {  
        println!("Staring actuator {:?}",self.name);  
        let mut last_admin_update = Instant::now();
        let mut last_message:Option<Instant> = None;
       
        loop {

            if let Ok(msg) = self.rx.try_recv() {
                println!("Staring actuator {:?}",self.name);  
                match msg.set_state {
                    State::CONTRACTING => {
                        self.contract_at(msg.speed);
                    },
                    State::EXPANDING => {
                        self.expand_at(msg.speed);
                    },
                    State::IDLE => {
                        self.stop();
                    }                    
                }
            }

            self.update();
            if last_admin_update.elapsed().as_secs() >= 1 {
                last_admin_update = Instant::now();
                println!("Admin update {}",self.pressure);
                let mut message = format!("SP{}",self.name).as_bytes().to_vec();
                message.extend(vec![0]);
                message.extend(self.pressure.to_le_bytes().to_vec());
                self.tx.send(message).unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
    fn contract_at(&mut self, speed: f32) {
        println!("Contracting at {}", speed);
        if speed == 0.0 {
            self.stop();
        } else {
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