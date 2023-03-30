import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn
from adafruit_motorkit import MotorKit
from time import sleep
from adafruit_servokit import ServoKit

skit = ServoKit(channels=16)

print("Actuator 1 - Increase flow (S1)")
skit.servo[0].angle = 0
sleep(2)
print("Stop")
skit.servo[0].angle = 99
sleep(2)
print("Actuator 1 - Decrease flow (S1)")
skit.servo[0].angle = 180
sleep(2)
print("Stop")
skit.servo[0].angle = 99
sleep(2)

print("Actuator 2 - Increase flow (S2)")
skit.servo[1].angle = 0
sleep(2)
print("Stop")
skit.servo[1].angle = 98.5
sleep(2)
print("Actuator 2 - Decrease flow (S2)")
skit.servo[1].angle = 180
sleep(2)
print("Stop")
skit.servo[1].angle = 98.5
sleep(2)
