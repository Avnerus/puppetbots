from adafruit_servokit import ServoKit
from time import sleep

kit = ServoKit(channels=16)

angle = 99
kit.servo[0].angle = angle
kit.servo[1].angle = angle

