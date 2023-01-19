use std::{thread};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::{Arc};
use std::collections::HashMap;
use std::str;

use adafruit_motorkit::{Motor};

mod actuator;
use self::actuator::{Actuator, ActuatorProps};
use Config;

fn intToMotorEnum(index: u16) -> Option<Motor> {
    match index {
        1 => Some(Motor::Motor1),
        2 => Some(Motor::Motor2),
        3 => Some(Motor::Motor3),
        4 => Some(Motor::Motor4),
        _ => None
    }
}

pub fn start(
    config: Arc<Config>,
    puppet_tx: mpsc::Sender<Vec<u8>>,
    server_rx: mpsc::Receiver<Vec<u8>>
) {

    let mut actuators: HashMap<String, Actuator> = HashMap::new();
    for actuator in &config.actuators {
        println!("Creating actutor {:?}", actuator.name);

        let actuator = match Actuator::new(
            ActuatorProps {
                name: actuator.name.clone(),
                pressure_i2c_dev: actuator.pressureDevice.clone(),
                contract_motor: intToMotorEnum(actuator.contractMotor).unwrap(),
                expand_motor: intToMotorEnum(actuator.expandMotor).unwrap()
            }
        ) {
            Ok(mut result) => {
                result.update();
                actuators.insert(result.name.to_string(), result);
            }
            Err(e) => {
                println!("Error initializing actuator: {:?} - {:?}", actuator.name, e);
            }
        };

    }

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

