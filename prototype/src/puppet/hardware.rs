#[cfg(not(target_os = "windows"))]
pub mod rpi_interface;

pub mod dummy_interface;

pub trait HardwareInterface {
    fn set_dc_motor(&mut self, index: u16, throttle:f32);    
    fn read_adc(&mut self, index:u16) -> i16;    
    fn set_servo_angle(&mut self, index:u16, angle: f32);        
}
