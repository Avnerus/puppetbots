extern crate ads1x1x;
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate nb;
extern crate adafruit_motorkit;
extern crate pwm_pca9685;

use ads1x1x::{channel, Ads1x1x, FullScaleRange};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};
use std::thread;
use std::time::{Duration};

use pwm_pca9685::{Pca9685, SlaveAddr, Channel};

fn main() {
    println!("Hello world!");


    let mut servo_kit = AdafruitServoKit::new();

    //153, 322

    //for n in (320..350).step_by(1) {
    //for n in [460, 153, 325] {
    for n in [0.0, 180.0, 100.5] {
        servo_kit.set_angle(Channel::C1, n);
        thread::sleep(Duration::from_secs(3));
   }


    //pwm.destroy();


    /*
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1115(dev, SlaveAddr::default());
    adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();


    let measurement = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", measurement);
    thread::sleep(Duration::from_secs(1));

    let _dev = adc.destroy_ads1115(); // get I2C device back>

    let mut pwm = init_pwm(None).unwrap();
    let mut pwm2 = init_pwm(None).unwrap();
    let mut dc_motor = DcMotor::try_new(&mut pwm, Motor::Motor1).unwrap();
    let mut dc_motor2 = DcMotor::try_new(&mut pwm2, Motor::Motor2).unwrap();

    println!("Motor1 on");
    dc_motor.set_throttle(&mut pwm, 1.0).unwrap();
    thread::sleep(Duration::from_secs(2));

    println!("Motor1 off");
    dc_motor.set_throttle(&mut pwm, 0.0).unwrap();

    dc_motor.stop(&mut pwm).unwrap();

    thread::sleep(Duration::from_secs(1));


    println!("Motor2 on");
    dc_motor2.set_throttle(&mut pwm2, 1.0).unwrap();
    thread::sleep(Duration::from_secs(1));

    println!("Motor2 off");
    dc_motor2.set_throttle(&mut pwm2, 0.0).unwrap();
    */
}


/*   
 * Adapted from multiple sources:
 * 1. https://github.com/ostrosco/adafruit_pwm_servo_driver/blob/master/src/servo_driver.rs
 * 2. https://github.com/ostrosco/adafruit_motorkit/blob/master/src/dc.rs
 * 3. https://github.com/adafruit/Adafruit_CircuitPython_ServoKit/blob/main/adafruit_servokit.py
 * 4. https://github.com/adafruit/Adafruit_CircuitPython_Motor/blob/main/adafruit_motor/servo.py
*/
struct AdafruitServoKit {
    min_duty: u16,
    duty_range: u16,
    pwm: Pca9685<I2cdev>
}

impl AdafruitServoKit {
    fn new() -> AdafruitServoKit {
        // Figures used by Adafruit CircuitPython
        let min_pulse = 750.0;
        let max_pulse = 2250.0;
        let frequency = 50.0;
        
        /* CircuitPython's pwmio supports 16bit precision, but pwm_pca9685 supports 12bit
         * which is also the frequency supported by Adafruit's servo bonnnet. 
         * duty figures are adjusted for 12bit (0xFFF = 4095) rather than 16bit (0xFFFF = 65535)
        */
        let min_duty = ((min_pulse * frequency) / 1000000.0 * 4095.0) as u16;
        let max_duty = ((max_pulse * frequency) / 1000000.0 * 4095.0) as u16;
        let duty_range = max_duty - min_duty;
        
        println!("Min duty: {:?}, Max duty: {:?}", min_duty, max_duty);

        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        /* 
         * Servo bonnet uses the I2C address 0x40. in I2C addresses, the first bit is always on and the last bit is r/w
         * so only the middle 6bits are specified
        */
        let address = SlaveAddr::Alternative(false, false, false, false, false, false); // 0x40 (0b1000000)

        // formula for prescale is update rate / duty range / frequency hz - 1
        let prescaleval:f32 = 25e6 / 4096.0 / frequency - 1.0;
        let prescale = (prescaleval + 0.5).floor() as u8;

        println!("Prescale: {:?}",prescale);

        let mut pwm = Pca9685::new(dev, address);
        pwm.enable().unwrap();
        pwm.set_prescale(prescale).unwrap();

        AdafruitServoKit {
           min_duty,
           duty_range,
           pwm
        }
    }
    fn set_angle(&mut self, channel:Channel, angle: f32) {
        // Assuming servo angle range of 0-180
        let duty_cycle = self.min_duty + ((angle / 180.0) * self.duty_range as f32) as u16;
        println!("Setting for angle {:?} duty cycle: {:?}",angle, duty_cycle);
        self.pwm.set_channel_on(channel, 0).unwrap();
        self.pwm.set_channel_off(channel, duty_cycle).unwrap();
    }
}

