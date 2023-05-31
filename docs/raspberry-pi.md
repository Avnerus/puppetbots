# Setting up the Raspberry Pi

First thing you need to do is to gain ssh access to the Raspberry Pi.

It can be done in many ways, but we suggest connecting an LCD display and keyboard.

1. Make sure raspberry is turned off.
2. Connect the display (it is inserted right into the HDMI port on the PI).
3. Connect the keyboard to the USB port.
4. Power up LCD display (it is done with MicroUSB cable).
5. Power up raspberry with the adapter from the kit.
6. Wait for terminal prompt to appear on the display. Now you should login to the board (ask us for the user and password).
7. Now type `sudo raspi-config`, then choose `system options` -> `wireless lan`. Ask us for the Wi-Fi details and apply the changes.
8. We need to get the board's IP address. To do it type `ifconfig` in the terminal. Look for wlan0 interface. There should be an ipv4 address like 192.168.43.* . Write down this address.
9. Turn off raspberry by typing `sudo poweroff`. Wait for screen to turn blue. Now you can disconnect the display and keyboard.
10. Turn on the Raspberry PI and wait for a couple of seconds so it boots. Now you can try connecting to it via ssh. Use Putty or VS Code bash terminal's `ssh` command if you on Windows or the `ssh` command in terminal on mac / linux. Enter the Raspberry Pi's address. 

Good, now you can control the board!

On the board you already have the repository with the project cloned. But you need to pull the last changes. Here's how:
```
cd puppetbots
git pull
```
