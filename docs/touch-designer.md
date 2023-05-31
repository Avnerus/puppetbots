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

Type `conda info --envs` to find the directories with environments, we will need them later.

## Selecting the active hand tracking interface
Once the environments have been created, activate the appropriate node to select the environment (see the steps below). We will provide you with Rokoko motion capture glove, but there's only 4 of them for 6 groups so you'll have to share them. For testing you can use Mediapipe which is software that does image recognition to capture hand motion. It can use the camera on your laptop to do that.

### Mediapipe
For `mediapipe`, activate the `mediapipe_env` node (its in the middle) by toggling the `Active` property on the right window, and deactivate the `rokoko_env` node.

Additionally, locate the `MediaPipe` (at the left bottom) node and make sure that `Cooking` is enabled (cooking is a cross toggle at the left of the square).

Now we need to write the path to conda environment that we obtained before to the script. Right click on `mediapip_env` and choose "edit contents". Update conda_envs_folder so it points to the "mediapipe" environment location. On windows be carefult to replace "\" in the path with "/" :)

After doing these changes you will need to restart TouchDesigner and re-open the project.

In the bottom of the interface choose Range Limit to be "loop" instead of "once" and push "⏵" symbol. If everything works as expected you should be seeing image from your camera in the node tree and if you put the finger in front of the camera it should recognize fingers and mark them with dots of different colors.

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