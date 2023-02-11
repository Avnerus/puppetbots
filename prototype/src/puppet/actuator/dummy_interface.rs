use std::error::Error;
use puppet::actuator::{ActuatorInterface};

pub struct DummyInterfaceProps {
    pub speed_factor: f32
}

pub struct DummyInterface {
    pressure: i16,
    speed_factor: f32,
    expand_speed: f32,
    contract_speed: f32
}
impl DummyInterface {
    pub fn new(props: DummyInterfaceProps) -> Result<Box<dyn ActuatorInterface>,Box<dyn Error>> {
        Ok(
            Box::new(DummyInterface {
                speed_factor: props.speed_factor,
                pressure: 0,
                expand_speed: 0.0,
                contract_speed: 0.0
            })
        )
    } 
}
impl ActuatorInterface for DummyInterface {
    fn contract_at(&mut self, speed: f32) {
        self.contract_speed = speed * self.speed_factor;
    }
    fn expand_at(&mut self, speed: f32) {
        self.expand_speed = speed * self.speed_factor;
    }
    fn stop(&mut self) {
        self.contract_speed = 0.0;
        self.expand_speed = 0.0;

    }
    fn read_pressure(&mut self) -> i16 {
        self.pressure
    }

    fn update(&mut self) {
        self.pressure = 
            self.pressure + 
            (self.contract_speed * self.speed_factor) as i16 - 
            (self.expand_speed * self.speed_factor) as i16
        ;
    }

}
