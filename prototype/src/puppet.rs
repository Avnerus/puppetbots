use std::{thread};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::collections::HashMap;
use std::str;

use adafruit_motorkit::{Motor};

mod actuator;
use self::actuator::{Actuator, ActuatorProps};

pub fn start(
    puppet_tx: mpsc::Sender<Vec<u8>>,
    server_rx: mpsc::Receiver<Vec<u8>>
) {

    let mut actuators: HashMap<String, Actuator> = HashMap::new();

    let mut test1 = Actuator::new(
        ActuatorProps {
            name: "Test",
            pressure_i2c_dev: "/dev/i2c-1",
            contract_motor: Motor::Motor1,
            expand_motor: Motor::Motor2
        }
    );

    test1.update();
    actuators.insert(test1.name.to_string(), test1);

    let mut last_admin_update = Instant::now();
    
    loop {
        if let Ok(msg) = server_rx.try_recv() {
            let command = msg[0] as char;
            match command {
                'M' => {
                    let null_pos = msg.iter().position(|&x| x == 0).unwrap();
                    let motor = str::from_utf8(&msg[1..null_pos]).unwrap();
                    let value = msg[null_pos + 1];
                    println!("Puppet motor command {}: {}", motor, value);

                    if let Some(actuator) = actuators.get_mut(&motor.to_string()) {
                        actuator.contract(value as f32 / 255.0);
                    } else {
                        println!("Unknown actuator {}", motor);
                    }
                }
                _ => {
                    println!("Unknown Motor command! {}",command);
                }
            }
        }
        for actuator in actuators.values_mut() {
            actuator.update();
            if last_admin_update.elapsed().as_secs() >= 1 {
                last_admin_update = Instant::now();
                println!("Admin update {}",actuator.pressure);
                let mut message = format!("SP{}",actuator.name).as_bytes().to_vec();
                message.extend(vec![0]);
                message.extend(actuator.pressure.to_le_bytes().to_vec());
                puppet_tx.send(message).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

