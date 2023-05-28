# Pneumatics Panel
## Assembly

### Raspberry Pi
The Raspberry Pi is the central controller of the telepuppet. 
1. Screw the raspberry pi to the _Rpi mount_ component. 
_If there is a screw shortage you can use only two screws out of the four holes available._ 
2. Mount the component in the center of the panel.

### Rotating plate
The plate is operated by an SG92R servo motor and is located outisde of the panel.
1. Screw the servo into the base using the provided mounting screws.
2. Attach the 'plus-shaped' horn to the servo, screw the horn using the provided small horn screw.
3. Connect the _Servo Bonnet_ to the raspbery pi as shown in this image:
![servo bonnet](servo-bonnet.jpg)

**Make sure the bonnet header is alligned correctly with the pins (no pins should remain outside of the bonnet header)**

4. Conncet the SG92R motor to the bonnet. The dark (brown or black) wire should be on the side that says **G**.
5. Mount the plate on the horns.
6. Mount the puppet's body onto the plate.

### Solenoid valves
The solenoid valves regulate the incoming and outgoing air from an actuator. 
1. Valve connectors scew into the valve. Before screwing a connector, wrap the threads for 2-3 layers of Teflon tape.
2. Inlet valve require two connectors, outlet valves just one.
3. Connect the _dc motor bonnet_ to the raspberry pi as shown in this image:

![dc bonnet](dc-bonnet.jpg)

_The DC bonnet can be stacked on top of the servo bonnet, or the pressure sensor hat_
4. Plug the solenoid valve's cables into the dc motor bonnet's screw terminals (M1, M2, etc. Ground terminal can remain empty).



## Test
The following scripts tests that all components are functional.

**Do not run the script when air flow is connected**

```
$ cd puppetbots/prototype
```
```
$ python3 util/connection_check.py
```


