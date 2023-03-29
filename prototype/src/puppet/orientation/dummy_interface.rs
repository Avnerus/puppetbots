use std::error::Error;
use crate::puppet::orientation::{OrientationInterface};

pub struct OrientationDummyInterface {
}

impl OrientationDummyInterface {
    pub fn new() -> Result<Box<dyn OrientationInterface + Send + Sync>,Box<dyn Error>> {
        println!("Creating dummy orientation interface");
        Ok(
            Box::new(OrientationDummyInterface {})
        )
    } 
}
impl OrientationInterface for OrientationDummyInterface {
    fn set_angle(&mut self, angle: f32) {
        println!("DUMMY: Set angle to {}", angle);
    }
}
