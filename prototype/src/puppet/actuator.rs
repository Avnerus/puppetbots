pub mod rpi_interface;

#[derive(PartialEq)]
pub enum State {
    CONTRACTING,
    EXPANDING,
    IDLE
}

pub struct ActuatorProps {
    pub name: String,
    pub interface: Box<dyn ActuatorInterface>
}

pub struct Actuator {
    pub name: String,
    pub pressure: i16,
    pub state: State,
    pub interface: Box<dyn ActuatorInterface>
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
            pressure: 0,
            state: State::IDLE
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
