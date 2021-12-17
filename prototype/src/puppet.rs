use std::{thread,time};
use std::time::{Duration, Instant};
use std::sync::{Arc,Mutex};
use std::sync::mpsc;

use linux_embedded_hal::I2cdev;
use ads1x1x::{
    Ads1x1x, 
    SlaveAddr, 
};

mod actuator;
use self::actuator::{Actuator, State};

pub fn start(
    puppet_tx: mpsc::Sender<Vec<u8>>
) {

    let mut test1 = Actuator {
        name: "Test",
        state: State::IDLE,
        pressure: 0,
        adc: Ads1x1x::new_ads1115(
            I2cdev::new("/dev/i2c-1").unwrap(),
            SlaveAddr::default()
        )
    };
    test1.init();

    let mut last_admin_update = Instant::now();
    
    loop {
        test1.update();
        if last_admin_update.elapsed().as_secs() >= 1 {
            last_admin_update = Instant::now();
            println!("Admin update {}",test1.pressure);
            let message = format!("SP{}",test1.name).as_bytes().to_vec();
            message.appendtest1.pressure.to_le_bytes().to_vec())()
            puppet_tx.send( + test1.pressure.to_le_bytes().to_vec()).unwrap();
        }
        thread::sleep(Duration::from_millis(100));
    }
}

