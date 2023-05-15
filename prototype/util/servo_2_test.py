from adafruit_servokit import ServoKit
from time import sleep

kit = ServoKit(channels=16)
servo_index = 1

print(kit)

angle = 0 
kit.servo[servo_index].angle = angle

while (True):
   kit.servo[servo_index].angle = angle
   sleep(1)
   angle = angle + 90
   if angle > 180:
       angle = 0

