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
use pwm_pca9685::Pca9685;
use std::error::Error;
use puppet::actuator::{ActuatorInterface};

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

pub struct RPIInterfaceProps {
    pub pressure_i2c_dev: String,
    pub contract_motor: Motor,
    pub expand_motor: Motor
}

pub struct RPIInterface {
    adc: Adc,
    contract_valve: DcMotor,
    contract_pwm: Pca9685<I2cdev>,
    expand_valve: DcMotor,
    expand_pwm: Pca9685<I2cdev>
}
impl RPIInterface {
    pub fn new(props: RPIInterfaceProps) -> Result<Box<dyn ActuatorInterface>,Box<dyn Error>> {
        let mut contract_pwm =  init_pwm(None)?;
        let mut expand_pwm =  init_pwm(None)?;
        
        let mut contract_valve = DcMotor::try_new(&mut contract_pwm, props.contract_motor)?;
        let mut expand_valve = DcMotor::try_new(&mut expand_pwm, props.expand_motor)?;

        expand_valve.set_throttle(&mut expand_pwm, 0.0)?;
        contract_valve.set_throttle(&mut contract_pwm, 0.0)?;

        let mut adc = Ads1x1x::new_ads1115(
            I2cdev::new(props.pressure_i2c_dev)?,
            SlaveAddr::default()
        );

        match adc.set_full_scale_range(FullScaleRange::Within4_096V) {
            Ok(()) => Ok(
                Box::new(RPIInterface {
                    adc,
                    contract_pwm,
                    expand_pwm,
                    contract_valve,
                    expand_valve
                })
            ),
            Err(e) => Err(format!("I2CError setting ADC range {:?}",e))?
        }
    } 
}
impl ActuatorInterface for RPIInterface {
    fn contract_at(&mut self, speed: f32) {

        self.contract_valve.set_throttle(&mut self.contract_pwm, speed).unwrap();
        self.expand_valve.set_throttle(&mut self.expand_pwm, 0.0).unwrap();

    }
    fn expand_at(&mut self, speed: f32) {
        self.expand_valve.set_throttle(&mut self.expand_pwm, speed).unwrap();
        self.contract_valve.set_throttle(&mut self.contract_pwm, 0.0).unwrap();
    }
    fn stop(&mut self) {
        self.expand_valve.set_throttle(&mut self.expand_pwm, 0.0).unwrap();
        self.contract_valve.set_throttle(&mut self.contract_pwm, 0.0).unwrap();
    }
    fn read_pressure(&mut self) -> i16 {
        block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()
    }

    fn update(&mut self) {
    }

}
