from adafruit_servokit import ServoKit
from time import sleep

kit = ServoKit(channels=16)

angle = 0
kit.servo[2].angle = angle

while (True):
   kit.servo[2].angle = angle
   sleep(0.5)
   angle = angle + 10 
   if angle >= 180:
       angle = 0
