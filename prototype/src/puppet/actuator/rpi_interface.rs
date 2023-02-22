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
use crate::puppet::actuator::{ActuatorInterface};

use crate::util::adafruit_servokit::{AdafruitServoKit};

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

pub struct RPIInterfaceProps {
    pub pressure_i2c_dev: String,
    pub inlet_motor: u16,
    pub outlet_motor: u16,
    pub flow_control_servo: u16
}

pub struct RPIInterface {
    adc: Adc,
    inlet_valve: DcMotor,
    inlet_pwm: Pca9685<I2cdev>,
    outlet_valve: DcMotor,
    outlet_pwm: Pca9685<I2cdev>,
    servo_kit: AdafruitServoKit,
    flow_control_channel: Channel
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
    pub fn new(props: RPIInterfaceProps) -> Result<Box<dyn ActuatorInterface + Send>,Box<dyn Error>> {
        let mut inlet_pwm =  init_pwm(None)?;
        let mut outlet_pwm =  init_pwm(None)?;
        
        let inlet_motor = int_to_motor_enum(props.inlet_motor).ok_or("Invalid motor index")?;
        let outlet_motor = int_to_motor_enum(props.outlet_motor).ok_or("Invalid motor index")?;

        let mut inlet_valve = DcMotor::try_new(&mut inlet_pwm, inlet_motor)?;
        let mut outlet_valve = DcMotor::try_new(&mut outlet_pwm, outlet_motor)?;

        inlet_valve.set_throttle(&mut inlet_pwm, 0.0)?;
        outlet_valve.set_throttle(&mut outlet_pwm, 0.0)?;

        let mut servo_kit = AdafruitServoKit::new();
        let flow_control_channel = int_to_channel(props.flow_control_servo).ok_or("Invalid servo index")?;

        let mut adc = Ads1x1x::new_ads1115(
            I2cdev::new(props.pressure_i2c_dev)?,
            SlaveAddr::default()
        );

        match adc.set_full_scale_range(FullScaleRange::Within4_096V) {
            Ok(()) => Ok(
                Box::new(RPIInterface {
                    adc,
                    inlet_pwm,
                    outlet_pwm,
                    inlet_valve,
                    outlet_valve,
                    servo_kit,
                    flow_control_channel
                })
            ),
            Err(e) => Err(format!("I2CError setting ADC range {:?}",e))?
        }

    } 
}
impl ActuatorInterface for RPIInterface {
    fn set_inlet_valve(&mut self, speed: f32) {
        self.inlet_valve.set_throttle(&mut self.inlet_pwm, speed).unwrap();
    }
    fn set_outlet_valve(&mut self, speed: f32) {
        self.outlet_valve.set_throttle(&mut self.outlet_pwm, speed).unwrap();
    }
    fn read_pressure(&mut self) -> i16 {
        block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()
    }
    fn start_flow_increase(&mut self) {
    }
    fn start_flow_decrease(&mut self) {
    }
    fn maintain_current_flow(&mut self) {
    }
    fn update(&mut self) {
    }
}
