use std::error::Error;
use crate::puppet::actuator::{ActuatorInterface};

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
    pub fn new(props: DummyInterfaceProps) -> Result<Box<dyn ActuatorInterface + Send>,Box<dyn Error>> {
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
    fn set_inlet_valve(&mut self, speed: f32) {
        self.contract_speed = speed * self.speed_factor;
    }
    fn set_outlet_valve(&mut self, speed: f32) {
        self.expand_speed = speed * self.speed_factor;
    }
    fn read_pressure(&mut self) -> i16 {
        self.pressure
    }
    fn start_flow_increase(&mut self) {
    }
    fn start_flow_decrease(&mut self) {
    }
    fn maintain_current_flow(&mut self) {
    }
    fn update(&mut self) {
        self.pressure = 
            self.pressure + 
            (self.contract_speed * self.speed_factor) as i16 - 
            (self.expand_speed * self.speed_factor) as i16
        ;
    }

}