# Rokoko Smart Gloves

# Installation
1. Install Rokoko Studio **Legacy** from [this link](https://www.rokoko.com/products/studio) (Under _Download Studio Legacy_).

2. Login with a user (ask Avner for a licensed user). 

3. If not exiting, create a project.

4. From the right panel, add an actor.

5. Connect a smart glove, when asked, configure it with the WiFi.

6. From the right tab drag the glove to the actor.

7. From the top menu click on "Perform calibration".

# Starting the stream
Start [Touch Designer](touch-designer.md) and activate the Rokoko environment per the instructions (restart TD if needed).

In rokoko studio, click on  "Start Livestream" on the right. Press the settings button for the final entry, configure the following settings:

 - Forward IP: `127.0.0.1`
 - Port: `14043`
 - Data format: `Json v3 Lz4`.

 Toogle the button to activate the livestream.

# Usage
- Data from the smart glove is collected to `rokoko_data` node.
 - The `glove_rotation_y` node collectes the data relevant to finger flexion and extension. To find the finger you are interested in, check which values change when you bend the finger.
 - By default, the controller listens to the left middle finger and left thumb.
 - Each finger triggers an `inlet_trigger` node when flexing and an `outlet_trigger` node when extending.
 - To fine-tune the sensitivty of the trigger, edit the `Trigger threshold` property of the node.
 - The `inlet_script` node sends a command to open the air inlet for the amount of time the trigger was activated.
 - It also sets the flow control valve according to the `bend_speed` node.
 - To adjust the conversion between the glove values and the flow speed, adjust the maximum value of the `bend_speed_clamp` node.
 - the `outlet_script` sends a command to open the air outlet for the amount of time the trigger was activated. It always opens in full speed.
