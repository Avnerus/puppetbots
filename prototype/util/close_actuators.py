import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_motorkit import MotorKit
from time import sleep

mkit = MotorKit()

print("Closing all inlets and outlets")

mkit.motor1.throttle = 0.0 
mkit.motor2.throttle = 0.0 
mkit.motor3.throttle = 0.0 
mkit.motor4.throttle = 0.0 
