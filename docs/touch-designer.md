# Touch Designer Controller
The touch designer controller mediates between a hand tracking interface and a [Rust Server](rust-server.md) controlling the puppet.

## Creating the python environments
Install Miniconda.

Create the `mediapipe` environmnt:
```
conda create -n mediapipe python=3.9
```
Activate it, and install the `mediapipe` package:
```
conda activate mediapipe
```
```
pip3 install mediapipe
```

Create the `rokoko` environment:
```
conda deactivate
```
```
conda create -n rokoko python=3.9
```
Activate it, and install the `lz4` package:
```
conda activate rokoko
```
```
pip3 install lz4
```

## Selecting the active hand tracking interface
Once the environments have been created, activate the appropriate node to select the environment.

### Mediapipe
For `mediapipe`, activate the `mediapipe_env` node by toggling the `Active` property on the right window, and deactivate the `rokoko_env` node.

Additionally, locate the `mediapipe` container node and make sure that `Cooking` is enabled.

### Rokoko smartgloves
For `rokoko` activate the `rokoko_env` node and deactivate the `mediapipe_env` node.

Additionally, locate the `mediapipe` container node and make sure that `Cooking` is disabled.

## Conneting to the Rust Server
1. Start the rust server, note the IP address of the server.
2. Select the `websocket1` node. Edit the IP in the `Network Address` property.
3. Click the `Reset` the button to initiate the connection to the server.

## Seeing pressure values.
The `pressure_values` node shows the current air pressure value of the configured actuators.

## Initiating custom motor commands
The _Motor coommabnd_ button at the top is used to send custom motor commands to the raspbery pi.

For example: To set the first servo to 45 degrees:
```
motor_command_motor_type: S
```
```
motor_command_motor_index: 1
```
```
motor_command_motor_value: 45
```

To open the solenoid connected at M2:
```
motor_command_motor_type: D
```
```
motor_command_motor_index: 2
```
```
motor_command_motor_value: 255
```
(To close the solenoid, set the value to `0` )