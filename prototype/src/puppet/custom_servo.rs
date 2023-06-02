use std::sync::{Arc, Mutex};

use crate::puppet::hardware::{HardwareInterface};

const MINIMUM_ANGLE_DIFFERENCE:f32 = 2.0;

pub struct CustomServoProps {
    pub interface: Arc<Mutex<Box<dyn HardwareInterface + Send + Sync>>>,
    pub servo_index: u16
}

pub struct CustomServo {
    pub current_angle:u8,
    pub servo_index: u16,
    pub interface: Arc<Mutex<Box<dyn HardwareInterface + Send + Sync>>>
}

pub trait CustomServoInterface {
    fn set_angle(&mut self, angle:f32);
}

pub struct CustomServoMessage {
    pub angle: f32
}

impl CustomServoMessage {
    pub fn set_state(angle: f32) -> ActuatorMessage {
        CustomServoMessage {
            angle
        }
    }
}

impl CustomServo {
    pub fn new(props: CustomServoProps) -> Self {
        CustomServo {
            interface: props.interface,
            servo_index: props.servo_index,
            current_angle: 0
        }
    }
    pub fn set_custom_servo_angle(&mut self, angle: u8) {
        if (angle as f32 - self.current_angle as f32).abs() >= MINIMUM_ANGLE_DIFFERENCE {
            println!("Custom servo: Set angle to {}", angle as f32);
            self.interface.lock().unwrap().set_servo_angle(self.servo_index, angle.into());
            self.current_angle = angle;
        }
    }
}
