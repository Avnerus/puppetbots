# Pneumatics Panel
## Assembly

### Panel bolts
Screw the panel bolts to the panel to have it lifted, so that the mountintg screws do not touch the bottom.

### Raspberry Pi
The Raspberry Pi is the central controller of the telepuppet. 
1. Screw the raspberry pi to the _Rpi mount_ component. 
_If there is a screw shortage you can use only two screws out of the four holes available._ 
2. Connect the _Servo Bonnet_ to the raspbery pi on the  as shown in this image:

![servo bonnet](servo-bonnet.jpg)

**Make sure the bonnet header is alligned correctly with the pins (no pins should remain outside of the bonnet header)**

3. Connect the _pressue sensing hat_ the servo bonnet.

    **Note: In some cases, the servo motor connectors push the hat out, causing the pins to disengage. In these cases, it helps to connect some servo motors in the left side of the bonnet and some in the right side of the bonnet.**

4. Connect the _dc motor bonnet_ to the _pressure sensing hat_ as shown in this image:

![dc bonnet](dc-bonnet.jpg)

5. Connect the **12V power adapter** to the dc motor bonnet.

6. Mount the raspberry pi in the center of the panel.


### Solenoid valves
The solenoid valves control the incoming and outgoing air from an actuator. They can be either fully open or fully closed.

1. Valve connectors screw into the valve. Before screwing a connector, wrap the threads for 2-3 layers of Teflon tape.
2. Inlet valve require two connectors, outlet valves just one.

3. Mount the solenoid valves to their apprproriate location on the panel.

4. Plug the solenoid valve's cables into the dc motor bonnet's screw terminals (M1, M2, etc. Ground terminal can remain empty).


### Servo valves
The servo valve regulates the flow of air entering the actuator proportionally. It is comprised of a manual ball valve and servo motor that moves the handle.

Here are the parts needed to assemble a single servo valve:

![pneumatics_panel-inventory.jpg](pneumatics_panel-inventory.jpg)

And here how it should look like when assembled:

![pneumatics_panel-assembled.jpg](pneumatics_panel-assembled.jpg)

### Calibrating the servo valve
The ball valve is fully open when the handle is at a parallel to the valve and is fully closed the handle is perpendicular to the valve.

The servo motor connects to the ball valve via a coupling adapter.

Servo motor is wired to controller which can send commands to set the angle to any value within 0..80 degrees range.

Servo horn should be attached to the servo in a way that when controller send "0" angle horn is in horizontal position like on the picture below:

![pneumatics_panel-calibrated.jpg](pneumatics_panel-calibrated.jpg)

Servo horn should be calibrated so that when the controller sends command to set rotation to 0 at the fully horizontal position, matching the ball valve's handle position.

So in zero 

1. Use the `connection_check_servo.py` script to move the servo motor to 0 degrees.
2. Now attach the horn to the servo, so it is in a position like on the photo above ^
3. Try moving the servo motor to 80 degrees with the same script, `connection_check_servo.py`. Horn now should be +-perpendicular to the valve.

### Assembling the servo valve
Once the servo is calibrated, the valve can be assembled.

1. Place the ball valve in its designated location and close the lid with the M3 screw.

2. Make sure the ball valve is open (handle is parallel to valve body). Place the coupling adapter on the handle (use the side with square groove, see photo below).

![pneumatics_panel-groove.jpg](pneumatics_panel-groove.jpg)

3. Mount the servo motor into the coupling adapter and secure it via the screws. **Do not screw the servo too tightly**

### Connecting the servo valve.

1. Connect the servo motors to the servo bonnet.

2. Mount the servo valves to their locations on the panel.

3. Use the `connection_check_servo.py` script to test the setup. 


### Cross adapters
Cross adapter split the incoming air flow to the actuator to three possible pathways:

1. The actuator itself.
2. A pressure sensor.
3. The outlet valve (to release the air).

Mount the cross adapters to their appropriate location on the panel.

## Tubing.

The air arrives from the compressor and is split to two actuators via the 10mm->6mm adapter.

Some connectors use a `Push-in` type. Simply push the tube into the port until the edge pops out the tube is locked. To release the tube, press the edge of the connector and then pull the tube out.

Other connetors (barbed plastic) fit inside the tube.

The tubing path is as follows:

1. A 6mm (outer diameter) tube from the adapter goes to an inlet valve.
2. A 6mm tube goes from the inlet valve to the servo valve.
3. A 6mm tube goes from the servo valve to the cross connetor.
4. A 6mm tube goes from the cross connector to the inlet valve.
5. A 6mm gube goes from the the cross conector to the actuator. Use a linear extender to bring the actuator further away from the cross connector.
6. A 6mm tube goes into a liner extender, which goes into a 3mm tube that goes into one pressure sensor.



## Test
The following scripts tests that all components are functional.

**Do not run the script when air flow is connected**

```
$ cd puppetbots/prototype
```
```
$ python3 util/connection_check.py
```


