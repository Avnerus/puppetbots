extern crate linux_embedded_hal;
extern crate pwm_pca9685;

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Pca9685, SlaveAddr, Channel};

/*   
 * PWM Control for ADAFruit Servo bonnet for Raspberry Pi
 *
 * Adapted from multiple sources:
 * 1. https://github.com/ostrosco/adafruit_pwm_servo_driver/blob/master/src/servo_driver.rs
 * 2. https://github.com/ostrosco/adafruit_motorkit/blob/master/src/dc.rs
 * 3. https://github.com/adafruit/Adafruit_CircuitPython_ServoKit/blob/main/adafruit_servokit.py
 * 4. https://github.com/adafruit/Adafruit_CircuitPython_Motor/blob/main/adafruit_motor/servo.py
*/
pub struct AdafruitServoKit {
    min_duty: u16,
    duty_range: u16,
    pwm: Pca9685<I2cdev>
}

impl AdafruitServoKit {
    pub fn new() -> AdafruitServoKit {
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
        
        let dev = I2cdev::new("/dev/i2c-1").unwrap();

        /* 
         * Servo bonnet uses the I2C address 0x40. in I2C addresses, the first bit is always on and the last bit is r/w
         * so only the middle 6bits are specified
        */
        let address = SlaveAddr::Alternative(false, false, false, false, false, false); // 0x40 (0b1000000)

        // formula for prescale is update rate / duty range / frequency hz - 1
        let prescaleval:f32 = 25e6 / 4096.0 / frequency - 1.0;
        let prescale = (prescaleval + 0.5).floor() as u8;

        let mut pwm = Pca9685::new(dev, address);
        pwm.enable().unwrap();
        pwm.set_prescale(prescale).unwrap();

        AdafruitServoKit {
           min_duty,
           duty_range,
           pwm
        }
    }
    pub fn set_angle(&mut self, angle: f32, channel:Channel) {
        // Assuming servo angle range of 0-180
        let duty_cycle = self.min_duty + ((angle / 180.0) * self.duty_range as f32) as u16;
        self.pwm.set_channel_on(channel, 0).unwrap();
        self.pwm.set_channel_off(channel, duty_cycle).unwrap();
    }
}

