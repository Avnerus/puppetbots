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

mod orientation;
use self::orientation::{Orientation, OrientationProps, OrientationInterface};
use self::orientation::dummy_interface::{OrientationDummyInterface};


use crate::soft_error::{SoftError};

use crate::Config;

pub fn start(
    config: Arc<Config>,
    puppet_tx: mpsc::Sender<Vec<u8>>,
    server_rx: mpsc::Receiver<Vec<u8>>
) {

    let mut actuators: HashMap<String, mpsc::Sender<actuator::ActuatorMessage>> = HashMap::new();
    for actuator in &config.actuators {
        println!("Creating actutor {:?}", actuator.name);

        let interface: Result<Box<dyn ActuatorInterface + Send + Sync>, Box<dyn Error>> = match actuator.interface_type.as_str() {
            #[cfg(not(target_os = "windows"))]
            "rpi" => {
                actuator::rpi_interface::RPIInterface::new(
                        actuator::rpi_interface::RPIInterfaceProps {
                        pressure_i2c_dev: actuator.pressure_device.clone(),
                        inlet_motor: actuator.inlet_motor,
                        outlet_motor: actuator.outlet_motor,
                        flow_control_servo: actuator.flow_control_servo
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
                let (actuator_tx, actuator_rx):
                 (mpsc::Sender<actuator::ActuatorMessage>, mpsc::Receiver<actuator::ActuatorMessage>) = mpsc::channel();

                let mut actuator = Actuator::new(
                    ActuatorProps {
                        name: actuator.name.clone(),
                        max_pressure: actuator.max_pressure,
                        flow_change_per_sec: actuator.flow_change_per_sec,
                        flow_stop_angle: actuator.flow_stop_angle,
                        interface: result as Box<dyn ActuatorInterface + Send + Sync>,
                        rx: actuator_rx,
                        tx: puppet_tx.clone()
                    }
                );
                actuators.insert(actuator.name.to_string(), actuator_tx);
                thread::Builder::new().name(actuator.name.to_owned()).spawn(move || {
                    actuator.start();        
                }).unwrap();
            }
            Err(e) => {
                println!("Error initializing actuator interface: {:?} - {:?}", actuator.name, e);
            }
        };
    }

    let orientation_interface: Result<Box<dyn OrientationInterface + Send + Sync>, Box<dyn Error>> =
    match config.orientation.interface_type.as_str() {
        #[cfg(not(target_os = "windows"))]
        "rpi" => {
            orientation::rpi_interface::OrientationRPIInterface::new(
                orientation::rpi_interface::OrientationRPIInterfaceProps {
                    orientation_servo: config.orientation.orientation_servo
                }
            )
        },
        "dummy" => {
            OrientationDummyInterface::new()
        },
        _ => Err(SoftError::new(format!("Invalid orientation interface type: {:?}",&config.orientation.interface_type).as_str()).into())
    };

    let mut orientation = Orientation::new(
        OrientationProps { 
            interface: orientation_interface.unwrap() as Box<dyn OrientationInterface + Send + Sync>
        }
    );

    loop {
        if let Ok(msg) = server_rx.try_recv() {
            let command = msg[0] as char;
            match command {
                'A' => {
                    let null_pos = msg.iter().position(|&x| x == 0).unwrap();
                    let motor = str::from_utf8(&msg[1..null_pos]).unwrap();
                    let motor_command = msg[null_pos + 1] as char;                   
                    println!("Puppet actuator command {}: {}", motor, motor_command);

                    if let Some(actuator_tx) = actuators.get_mut(&motor.to_string()) {
                        match motor_command {
                            'C' => {
                                let speed = msg[null_pos + 2];
                                let delay:u16 = u16::from_le_bytes([msg[null_pos + 3], msg[null_pos + 4]]);

                                println!("Speed: {:?}", speed);
                                println!("Delay: {:?}", delay);


                                actuator_tx.send(
                                    actuator::ActuatorMessage::set_state (
                                        actuator::State::Contracting,
                                        speed as f32 / 255.0,
                                        delay
                                    )
                                ).unwrap();                   
                            }
                            'E' => {
                                let speed = msg[null_pos + 2];
                                let delay:u16 = u16::from_le_bytes([msg[null_pos + 3], msg[null_pos + 4]]);

                                actuator_tx.send(
                                    actuator::ActuatorMessage::set_state (
                                        actuator::State::Expanding,
                                        speed as f32 / 255.0,
                                        delay
                                    )
                                ).unwrap(); 
                            
                            }
                            'S' => {
                                actuator_tx.send(
                                    actuator::ActuatorMessage::set_state (
                                        actuator::State::Idle,
                                        1.0,
                                        0
                                    )
                                ).unwrap();                         
                            },
                            'R' => {
                                actuator_tx.send(
                                    actuator::ActuatorMessage::set_state (
                                        actuator::State::FlowReset,
                                        1.0,
                                        0
                                    )
                                ).unwrap();                         
                            }
                            _ => {
                                println!("Unknown actuator motor command! {}", motor_command);
                            }
                        }
                    } else {
                        println!("Unknown actuator {}", motor);
                    }
                },
                'C' => {                    /*
                    let config_json = str::from_utf8(&msg[4..]).unwrap();   
                    let new_config:Config = serde_json::from_str(&config_json).unwrap();      
                                  
                    for actuator in &new_config.actuators {
                        if let Some(actuator_tx) = actuators.get_mut(&actuator.name) {
                            actuator_tx.send(
                                actuator::ActuatorMessage::set_config (
                                    actuator::ConfigMessage::MaxPressure,
                                    actuator.max_pressure as f32
                            )).unwrap();
                            actuator_tx.send(
                                actuator::ActuatorMessage::set_config (
                                    actuator::ConfigMessage::FlowChangePerSec,
                                    actuator.flow_change_per_sec
                            )).unwrap();
                        }
                    }*/
                },
                'O' => {
                    let angle = msg[1];
                    orientation.set_orientation_angle(angle.into());
                }
                _ => {
                    println!("Unknown puppet command! {}",command);
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

