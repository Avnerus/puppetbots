extern crate ads1x1x;
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate nb;
extern crate adafruit_motorkit;
extern crate pwm_pca9685;

use ads1x1x::{
    channel, 
    Ads1x1x, 
    FullScaleRange,
    interface::I2cInterface,
    SlaveAddr,
    ic::{Ads1115, Resolution16Bit} 
};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};
use pwm_pca9685::{Pca9685, Channel};
use std::error::Error;

use crate::puppet::hardware::{HardwareInterface};
use crate::util::adafruit_servokit::{AdafruitServoKit};

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

pub struct RPIInterfaceProps {
    pub i2c_dev: String    
}

pub struct RPIInterface {
    adc: Adc,
    dc_motors: Vec<DcMotor>,
    dc_pwms: Vec<Pca9685<I2cdev>>,    
    servo_kit: AdafruitServoKit    
}

fn int_to_motor_enum(index: u16) -> Option<Motor> {
    match index {
        1 => Some(Motor::Motor1),
        2 => Some(Motor::Motor2),
        3 => Some(Motor::Motor3),
        4 => Some(Motor::Motor4),
        _ => None
    }
}

fn int_to_channel(index: u16) -> Option<Channel> {
    match index {
        1 => Some(Channel::C0),
        2 => Some(Channel::C1),
        3 => Some(Channel::C2),
        4 => Some(Channel::C3),
        _ => None
    }
}

impl RPIInterface {
    pub fn new(props: RPIInterfaceProps) -> Result<Box<dyn HardwareInterface + Send + Sync>,Box<dyn Error>> {
        let mut dc_motors = Vec::new();
        let mut dc_pwms = Vec::new();

        for i in 0..4 {
            let mut pwm = init_pwm(None)?;
            let motor = int_to_motor_enum(i as u16 + 1).ok_or("Invalid motor index")?;
            let dc_motor = DcMotor::try_new(&mut pwm, motor)?;
            dc_motors.push(dc_motor);
            dc_pwms.push(pwm);
        }

        let servo_kit = AdafruitServoKit::new();        

        let mut adc = Ads1x1x::new_ads1115(
            I2cdev::new(props.i2c_dev)?,
            SlaveAddr::default()
        );

        match adc.set_full_scale_range(FullScaleRange::Within4_096V) {
            Ok(()) => Ok(
                Box::new(RPIInterface {
                    adc,
                    dc_motors,
                    dc_pwms,        
                    servo_kit                    
                })
            ),
            Err(e) => Err(format!("I2CError setting ADC range {:?}",e))?
        }

    } 
}
impl HardwareInterface for RPIInterface {
    fn set_dc_motor(&mut self, index: u16, speed: f32) {
        // Motor indexes are 1-based
        self.dc_motors[index as usize - 1].set_throttle(&mut self.dc_pwms[index as usize], speed).unwrap();
    }
    fn set_servo_angle(&mut self, index:u16, angle: f32) {
        let channel = int_to_channel(index).unwrap();
        self.servo_kit.set_angle(angle, channel);
    } 
    fn read_adc(&mut self, index:u16) -> i16 {
        match index {
            1 => {
                block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()
            },
            2 => {
                block!(self.adc.read(&mut channel::DifferentialA2A3)).unwrap()
            },
            _ => {
                panic!("Invalid ADC index")
            }
        }
    }
}
