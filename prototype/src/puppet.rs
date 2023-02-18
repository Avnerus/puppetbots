use std::{thread};
use std::time::{Duration};
use std::sync::mpsc;
use std::sync::{Arc};
use std::collections::HashMap;
use std::str;
use std::error::Error;

mod actuator;
use self::actuator::{Actuator, ActuatorProps, ActuatorInterface};
use self::actuator::dummy_interface::{DummyInterface, DummyInterfaceProps};

use soft_error::{SoftError};

use Config;

pub fn start(
    config: Arc<Config>,
    puppet_tx: mpsc::Sender<Vec<u8>>,
    server_rx: mpsc::Receiver<Vec<u8>>
) {

    let mut actuators: HashMap<String, Actuator> = HashMap::new();
    for actuator in &config.actuators {
        println!("Creating actutor {:?}", actuator.name);

        let interface: Result<Box<dyn ActuatorInterface>, Box<dyn Error>> = match actuator.interface_type.as_str() {
            #[cfg(not(target_os = "windows"))]
            "rpi" => {
                actuator::rpi_interface::RPIInterface::new(
                        actuator::rpi_interface::RPIInterfaceProps {
                        pressure_i2c_dev: actuator.pressure_device.clone(),
                        contract_motor: actuator.contract_motor,
                        expand_motor: actuator.expand_motor
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
                let (actuator_tx, actuator_rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

                let mut actuator = Actuator::new(
                    ActuatorProps {
                        name: actuator.name.clone(),
                        interface: result,
                        rx: actuator_rx,
                        tx: puppet_tx.clone()
                    }
                );
                actuator.start();
                actuators.insert(actuator.name.to_string(), actuator);
            }
            Err(e) => {
                println!("Error initializing actuator interface: {:?} - {:?}", actuator.name, e);
            }
        };

    }

    loop {
        if let Ok(msg) = server_rx.try_recv() {
            let command = msg[0] as char;
            match command {
                'A' => {
                    let null_pos = msg.iter().position(|&x| x == 0).unwrap();
                    let motor = str::from_utf8(&msg[1..null_pos]).unwrap();
                    let motor_command = msg[null_pos + 1] as char;                   
                    println!("Puppet motor command {}: {}", motor, motor_command);

                    if let Some(actuator) = actuators.get_mut(&motor.to_string()) {
                        match motor_command {
                            'C' => {
                                let value = msg[null_pos + 2];
                                actuator.contract_at(value as f32 / 255.0);
                            }
                            'E' => {
                                let value = msg[null_pos + 2];
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
        thread::sleep(Duration::from_millis(100));
    }
}

