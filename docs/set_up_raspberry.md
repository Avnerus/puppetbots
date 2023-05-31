# Setting up raspberry

First thing you need to do is to gain ssh access to the Raspberry.

It can be done in many ways, but we suggest you connecting the LCD display / keyboard that we will provide to you.

1. Make sure raspberry is turned off
2. Connect the display (it is inserted right into the HDMI port on the PI)
3. Connect the keyboard to the USB port
4. Power up LCD display (it is done with MicroUSB cable)
5. Power up raspberry with the adapter from the kit
6. Wait for terminal prompt to appear on the display. Now you should login to the board, use login `pi` and password `tech2peace`
7. Now type `sudo raspi-config`, then choose `system options` -> `wireless lan`. Here you need to type wi-fi details, SSID is `craftoola` and password is `12345678`. Apply changes
8. Now we need to get the board's IP address. To do it type `sudo ifconfig` in the terminal. Look for wlan0 interface. There should be an ipv4 address like 192.168.43.* . Write down this address
9. Now turn off raspberry by typing `sudo poweroff`. Wait for screen to turn blue. Now you can disconnect display and keyboard and give it to other team so they setup their boards as well
10. Turn on the Raspberry PI and wait for a couple of seconds so it boots and connects to wi-fi. Now you can try connecting to it via ssh. Use Putty if you on Windows or terminal on mac / linux. Address you need to connect to is `<raspberry_ip>:22`. Login/password are the same as on the step 6

Good, now you can control the board!

On the board you already have the repository with the project cloned. But you need to pull the last changes. Here's how:
```
cd puppetbots
git pull
```
