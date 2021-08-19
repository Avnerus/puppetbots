use std::{thread,time};
use std::time::Duration;
use std::sync::{Arc,Mutex};
use std::sync::mpsc;
use ads1x1x::{
    channel, 
    Ads1x1x, 
    SlaveAddr, 
    FullScaleRange,
    interface::I2cInterface,
    ic::{Ads1115, Resolution16Bit} 
};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

enum State {
    DEFLATING,
    INFLATING,
    IDLE
}

struct Actuator {
    pressure: i16,
    state: State,
    adc: Adc
}

const MAX_PRESSURE: i16 = 100;

impl Actuator {
    fn init(&mut self) {
        self.adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();
    }
    fn deflate(&mut self) {
        match self.state {
            State::IDLE => {
                println!("Deflating!");
                self.state = State::DEFLATING;
            },
            _ => ()
        }
    }
    fn update(&mut self) {
        self.pressure = block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap();
        if self.pressure > MAX_PRESSURE {
            println!("Pressure surpassed MAX: {}", self.pressure);
            self.deflate();
        }
    }
}

pub fn start(
    actuator_rx: mpsc::Receiver<Vec<u8>>
) {

    let mut test1 = Actuator {
        state: State::IDLE,
        pressure: 0,
        adc: Ads1x1x::new_ads1115(
            I2cdev::new("/dev/i2c-1").unwrap(),
            SlaveAddr::default()
        )
    };
    test1.init();
    
    loop {
        if let Ok(msg) = actuator_rx.try_recv() {
            println!("Actuator msg! {:?}", msg);
        }
        test1.update();
        thread::sleep(Duration::from_millis(100));
    }
}

