import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn
from adafruit_motorkit import MotorKit
from time import sleep
from adafruit_servokit import ServoKit

skit = ServoKit(channels=16)
mkit = MotorKit()

i2c = busio.I2C(board.SCL, board.SDA)
ads = ADS.ADS1115(i2c)

chan1 = AnalogIn(ads, ADS.P0, ADS.P1)
chan2 = AnalogIn(ads, ADS.P2, ADS.P3)

print("Actuator 1 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan1.value))
    sleep(1)

print("Actuator 1 - Open outlet (M2)")
mkit.motor2.throttle = 1.0 
sleep(5)
print("Close")
mkit.motor2.throttle = 0.0 

print("Actuator 1 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan1.value))
    sleep(1)

print("Actuator 2 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan2.value))
    sleep(1)



print("Actuator 2 - Open outlet (M4)")
mkit.motor4.throttle = 1.0 
sleep(5)
print("Close")
mkit.motor4.throttle = 0.0 

print("Actuator 2 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan2.value))
    sleep(1)
