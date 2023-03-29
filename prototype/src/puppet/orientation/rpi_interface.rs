use std::error::Error;
use crate::puppet::orientation::{OrientationInterface};
use crate::util::adafruit_servokit::{AdafruitServoKit};
use pwm_pca9685::{Channel};

pub struct OrientationRPIInterfaceProps {
    pub orientation_servo: u16
}

pub struct OrientationRPIInterface {
    servo_kit: AdafruitServoKit,
    orientation_channel: Channel
}

// TODO: Redundant
fn int_to_channel(index: u16) -> Option<Channel> {
    match index {
        1 => Some(Channel::C0),
        2 => Some(Channel::C1),
        3 => Some(Channel::C2),
        4 => Some(Channel::C3),
        _ => None
    }
}

impl OrientationRPIInterface {
    pub fn new(props: OrientationRPIInterfaceProps) -> Result<Box<dyn OrientationInterface + Send + Sync>,Box<dyn Error>> {

        let servo_kit = AdafruitServoKit::new();
        let orientation_channel = int_to_channel(props.orientation_servo).ok_or("Invalid orientation servo index")?;
        
        Ok(Box::new(OrientationRPIInterface {
            servo_kit,
            orientation_channel
        }))
    } 
}
impl OrientationInterface for OrientationRPIInterface {   
    fn set_angle(&mut self, angle: f32) {
        self.servo_kit.set_angle(angle, self.orientation_channel);
    }    
}
