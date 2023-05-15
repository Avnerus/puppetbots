from adafruit_servokit import ServoKit
from time import sleep
from adafruit_motorkit import MotorKit

kit = ServoKit(channels=16)
servo_index = 0

mkit = MotorKit()

print(kit)

print("Actuator 1 - Open inlet (M1)")
mkit.motor1.throttle = 1.0

angle = 0 
kit.servo[servo_index].angle = angle

while (angle <= 90):
   kit.servo[servo_index].angle = angle
   sleep(1)
   angle = angle + 10


kit.servo[servo_index].angle = 0

print("Actuator 1 - Close")
mkit.motor1.throttle = 0.0
