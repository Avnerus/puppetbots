import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn
from time import sleep

i2c = busio.I2C(board.SCL, board.SDA)
ads = ADS.ADS1115(i2c)

chan1 = AnalogIn(ads, ADS.P0, ADS.P1)
chan2 = AnalogIn(ads, ADS.P2, ADS.P3)

print("Actuator 1 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan1.value))
    sleep(1)

print("Actuator 2 - Pressure readout")
for n in range(5):
    print("Pressure: {}".format(chan2.value))
    sleep(1)

