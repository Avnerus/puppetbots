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
    pub fn new(props: DummyInterfaceProps) -> Result<Box<dyn ActuatorInterface + Send + Sync>,Box<dyn Error>> {
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
        println!("Dummy setting inlet valve: {}", speed)
    }
    fn set_outlet_valve(&mut self, speed: f32) {
        println!("Dummy setting outlet valve: {}", speed)
    }
    fn read_pressure(&mut self) -> i16 {
        self.pressure
    }
    fn set_flow_angle(&mut self, angle: f32) {
        println!("Dummy setting flow angle: {}", angle)
    }
    fn update(&mut self) {
        self.pressure = 
            self.pressure + 
            (self.contract_speed * self.speed_factor) as i16 - 
            (self.expand_speed * self.speed_factor) as i16
        ;
    }

}
