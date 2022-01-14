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

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

#[derive(PartialEq)]
pub enum State {
    CONTRACTING,
    EXPANDING,
    IDLE
}

pub struct ActuatorProps {
    pub name: &'static str,
    pub pressure_i2c_dev: &'static str,
    pub contract_motor: Motor,
    pub expand_motor: Motor
}

pub struct Actuator {
    pub name: &'static str,
    pub pressure: i16,
    pub state: State,
    adc: Adc,
    contract_valve: DcMotor,
    contract_pwm: Pca9685<I2cdev>,
    expand_valve: DcMotor,
    expand_pwm: Pca9685<I2cdev>
}

const MAX_PRESSURE: i16 = 1000;
const TARGET_PRESSURE: i16 = 500;

impl Actuator {
    pub fn new(props: ActuatorProps) -> Actuator {
        let mut contract_pwm =  init_pwm(None).unwrap();
        let mut expand_pwm =  init_pwm(None).unwrap();
        
        let mut contract_valve = DcMotor::try_new(&mut contract_pwm, props.contract_motor).unwrap();
        let mut expand_valve = DcMotor::try_new(&mut expand_pwm, props.expand_motor).unwrap();

        expand_valve.set_throttle(&mut expand_pwm, 0.0).unwrap();
        contract_valve.set_throttle(&mut contract_pwm, 0.0).unwrap();

        let mut new_actuator = Actuator {
            name: props.name,
            pressure: 0,
            adc : Ads1x1x::new_ads1115(
                I2cdev::new(props.pressure_i2c_dev).unwrap(),
                SlaveAddr::default()
            ),
            contract_pwm: contract_pwm,
            expand_pwm: expand_pwm,
            contract_valve: contract_valve,
            expand_valve: expand_valve,
            state: State::IDLE,
        };
        new_actuator.adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();
        return new_actuator;
    } 
    pub fn contract(&mut self, speed: f32) {
        println!("Contracting at {}", speed);
        self.state = State::CONTRACTING;

        self.contract_valve.set_throttle(&mut self.contract_pwm, speed).unwrap();
        self.expand_valve.set_throttle(&mut self.expand_pwm, 0.0).unwrap();
       // self.expand_valve.stop(&mut self.expand_pwm).unwrap();

        if speed == 0.0 {
            println!("Stopping");
            //self.contract_valve.stop(&mut self.expand_pwm).unwrap();
            self.state = State::IDLE;
        }
    }
    pub fn expand(&mut self, speed: f32) {
        println!("Expanding at {}!",speed);
        self.state = State::EXPANDING;
        self.expand_valve.set_throttle(&mut self.expand_pwm, speed).unwrap();
        self.contract_valve.set_throttle(&mut self.contract_pwm, 0.0).unwrap();
      //  self.contract_valve.stop(&mut self.contract_pwm).unwrap();
    
        if speed == 0.0 {
            println!("Stopping");
          //  self.expand_valve.stop(&mut self.expand_pwm).unwrap();
            self.state = State::IDLE;
        }
    }
    pub fn stop(&mut self) {
        println!("Stopping");
        self.state = State::IDLE;
        self.expand_valve.set_throttle(&mut self.expand_pwm, 0.0).unwrap();
        self.contract_valve.set_throttle(&mut self.contract_pwm, 0.0).unwrap();
    }
    pub fn update(&mut self) {
        self.pressure = block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap();
        if self.pressure > MAX_PRESSURE && self.state != State::EXPANDING {
            println!("Pressure surpassed MAX: {}", self.pressure);
            self.expand(1.0);
        } else if self.pressure >= TARGET_PRESSURE && self.state == State::CONTRACTING {
            println!("Reached target pressure: {}", self.pressure);
            self.stop();
        }
    }
}
