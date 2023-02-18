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

pub struct ActuatorProps {
    pub name: String,
    pub interface: Box<dyn ActuatorInterface>,
    pub rx: mpsc::Receiver<Vec<u8>>,
    pub tx: mpsc::Sender<Vec<u8>>
}

pub struct Actuator {
    pub name: String,
    pub pressure: i16,
    pub state: State,
    pub interface: Box<dyn ActuatorInterface>,
    rx: mpsc::Receiver<Vec<u8>>,
    tx: mpsc::Sender<Vec<u8>>
}

pub trait ActuatorInterface {
    fn contract_at(&mut self, speed:f32);
    fn expand_at(&mut self, speed:f32);
    fn stop(&mut self);
    fn read_pressure(&mut self) -> i16;
    fn update(&mut self);
}

const MAX_PRESSURE: i16 = 1000;
const TARGET_PRESSURE: i16 = 500;

impl Actuator {
    pub fn new(props: ActuatorProps) -> Self {
        Actuator {
            name: props.name,
            interface: props.interface,
            rx: props.rx,
            tx: props.tx,
            pressure: 0,
            state: State::IDLE
        }
    } 
    pub fn start(&mut self) {  
        println!("Staring actuator {:?}",self.name);  
        let mut last_admin_update = Instant::now();
       
        loop {

            if let Ok(msg) = self.rx.try_recv() {
                println!("Actuator command!");
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
}
impl ActuatorInterface for Actuator {

    fn contract_at(&mut self, speed: f32) {
        println!("Contracting at {}", speed);
        self.state = State::CONTRACTING;

        self.interface.contract_at(speed);

        if speed == 0.0 {
            self.stop();
        }
    }
    fn expand_at(&mut self, speed: f32) {
        println!("Expanding at {}!",speed);
        self.state = State::EXPANDING;

        self.interface.expand_at(speed);

        if speed == 0.0 {
            println!("Stopping");
            self.stop();
        }
    }
    fn stop(&mut self) {
        println!("Stopping");
        self.interface.stop();
        self.state = State::IDLE;
    }
    fn update(&mut self) {
        self.pressure = self.read_pressure();
        if self.pressure > MAX_PRESSURE && self.state != State::EXPANDING {
            println!("Pressure surpassed MAX: {}", self.pressure);
            self.interface.expand_at(1.0);
        } else if self.pressure >= TARGET_PRESSURE && self.state == State::CONTRACTING {
            println!("Reached target pressure: {}", self.pressure);
            self.interface.stop();
        }
    }
    fn read_pressure(&mut self) -> i16 {
        self.interface.read_pressure()
    }
}
