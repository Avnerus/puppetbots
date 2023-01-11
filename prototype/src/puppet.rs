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

    let actuator = match Actuator::new(
        ActuatorProps {
            name: "Test",
            pressure_i2c_dev: "/dev/i2c-1",
            contract_motor: Motor::Motor1,
            expand_motor: Motor::Motor2
        }
    ) {
        Ok(mut test1) => {
            test1.update();
            actuators.insert(test1.name.to_string(), test1);
        }
        Err(e) => {
            println!("Error initializing actuator: {:?}", e);
        }
    };

    let mut last_admin_update = Instant::now();
    
    loop {
        if let Ok(msg) = server_rx.try_recv() {
            let command = msg[0] as char;
            match command {
                'A' => {
                    let null_pos = msg.iter().position(|&x| x == 0).unwrap();
                    let motor = str::from_utf8(&msg[1..null_pos]).unwrap();
                    let motor_command = msg[null_pos + 1] as char;
                    let value = msg[null_pos + 2];
                    println!("Puppet motor command {}: {}, {}", motor, motor_command, value);

                    if let Some(actuator) = actuators.get_mut(&motor.to_string()) {
                        match motor_command {
                            'C' => {
                                actuator.contract(value as f32 / 255.0);
                            }
                            'E' => {
                                actuator.expand(value as f32 / 255.0);
                            }
                            'S' => {
                                actuator.stop();
                            }
                            _ => {
                                println!("Unknown actuator motor command! {}", motor_command);
                            }
                        }
                    } else {
                        println!("Unknown actuator {}", motor);
                    }
                }
                _ => {
                    println!("Unknown puppet command! {}",command);
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

