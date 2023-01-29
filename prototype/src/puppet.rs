use std::{thread};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::{Arc};
use std::collections::HashMap;
use std::str;
use std::error::Error;

use adafruit_motorkit::{Motor};

mod actuator;
use self::actuator::{Actuator, ActuatorProps, ActuatorInterface};
use self::actuator::rpi_interface::{RPIInterface, RPIInterfaceProps};
use self::actuator::dummy_interface::{DummyInterface, DummyInterfaceProps};

use soft_error::{SoftError};

use Config;

fn int_to_motor_enum(index: u16) -> Option<Motor> {
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

        let interface: Result<Box<dyn ActuatorInterface>, Box<dyn Error>> = match actuator.interface_type.as_str() {
            "rpi" => {
                RPIInterface::new(
                        RPIInterfaceProps {
                        pressure_i2c_dev: actuator.pressure_device.clone(),
                        contract_motor: int_to_motor_enum(actuator.contract_motor).unwrap(),
                        expand_motor: int_to_motor_enum(actuator.expand_motor).unwrap()
                    }
                )
            },
            "dummy" => {
                DummyInterface::new(
                    DummyInterfaceProps { 
                        speed_factor: actuator.speed_factor
                    }
                )
            },
            _ => Err(SoftError::new(format!("Invalid actuator interface type: {:?}",actuator.interface_type).as_str()).into())
        };

        match interface {
            Ok(result) => {
                let mut actuator = Actuator::new(
                    ActuatorProps {
                        name: actuator.name.clone(),
                        interface: result
                    }
                );
                actuator.update();
                actuators.insert(actuator.name.to_string(), actuator);
            }
            Err(e) => {
                println!("Error initializing actuator interface: {:?} - {:?}", actuator.name, e);
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
                                actuator.contract_at(value as f32 / 255.0);
                            }
                            'E' => {
                                actuator.expand_at(value as f32 / 255.0);
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

