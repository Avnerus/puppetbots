# Rust server

## Configuring the server
```
$ cd puppetbots/prototype
```
Edit the config file `config-rpi.json`.

For each actuator, there is an `actuator` entry with the following configuration values:

1. `name`: The name used for commands passed through the socket interface via the [Touch designer controller]('touch-designer.md').
2. `pressureDeviceIndex`: The pressure sensor on the sensor board that is connected to the actuator. When viewed from the perspective of the ADS1105 controller at the bottom, the pressure sensor on a the top is `1` and the pressure sensor at the bottom is `2`.
3. `maxPressure`: The paximum pressure value allowed for this actuator. Beyond this pressure value the actuator will not inflate.
4. `flowChangeTimeMs`: How much time (ms) to wait for the servo valve to switch positions (default: `200ms`).
5. `flowMaxAngle`: The maximum angle in which the servo can rotate to **switch off** the flow on the ball valve (default: `80`).
6. `flowControlServo`: The index of the servo that is connected to the servo valve for this actuator. Index is 1-based.
7. `inletMotor`: The solenoid valve motor index that is the air  inlet for this actuator. `M1` corresponds to `1`, etc. 
8. `outletMotor`: The solenoid valve motor index that is the air outlet for this actuator.

Additionally, the following properties can be configured:
1. `server.port`: The port of the socket server, used to connect from the [Touch designer controller]('touch-designer.md').
2. `interfaceType`: Either `"rpi"` for Raspberry Pi or `"dummy"` for a non-unix environment.
3. `orientationServo`: The index of the sero controlling the puppet rotation plate. Index is 1-based.    


## Starting the server
```
$ cd puppetbots/prototype
```
```
$ cargo run config-rpi.json
```
If the `cargo` command is not found, install Rust via the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
