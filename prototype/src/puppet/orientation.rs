use crate::puppet::hardware::{HardwareInterface};

const MINIMUM_ANGLE_DIFFERENCE:f32 = 2.0;

pub struct OrientationProps {
    pub interface: Arc<Mutex<Box<dyn HardwareInterface + Send + Sync>>>,
    pub servo_index: u16
}

pub struct Orientation {
    pub current_angle:u8,
    pub servo_index: u16,
    pub interface: Arc<Mutex<Box<dyn HardwareInterface + Send + Sync>>>
}

pub trait OrientationInterface {
    fn set_angle(&mut self, angle:f32);        
}

impl Orientation {
    pub fn new(props: OrientationProps) -> Self {
        Orientation {            
            interface: props.interface,           
            current_angle: 0
        }
    }   
    pub fn set_orientation_angle(&mut self, angle: u8) {
        if (angle as f32 - self.current_angle as f32).abs() >= MINIMUM_ANGLE_DIFFERENCE {
            println!("Orientation: Set angle to {}", angle);
            self.interface.lock().unwrap().set_servo_angle(self.servo_index, angle.into());
            self.current_angle = angle;
        }
    }
}
