use std::error::Error;
use crate::puppet::hardware::{HardwareInterface};

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
    pub fn new(props: DummyInterfaceProps) -> Result<Box<dyn HardwareInterface + Send + Sync>,Box<dyn Error>> {
        Ok(
            Box::new(DummyInterface {
                speed_factor: 1.0,
                pressure: 0,
                expand_speed: 0.0,
                contract_speed: 0.0
            })
        )
    } 
    fn update(&mut self) {
        self.pressure = 
            self.pressure + 
            (self.contract_speed * self.speed_factor) as i16 - 
            (self.expand_speed * self.speed_factor) as i16
        ;
    }
}
impl HardwareInterface for DummyInterface {
    fn set_dc_motor(&mut self, index: u16, speed: f32) {
        println!("Dummy setting dc motor: {} to speed {}", index, speed);
    }
    fn set_servo_angle(&mut self, index:u16, angle: f32) {
        println!("Dummy setting servo: {} to angle {}", index, angle);
    } 
    fn read_adc(&mut self, index:u16) -> i16 {
       return pressure;
    }   
}
