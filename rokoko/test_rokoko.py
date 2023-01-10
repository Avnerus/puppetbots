from adafruit_motorkit import MotorKit
from time import sleep
from adafruit_servokit import ServoKit
import socket
import lz4.frame as lz4f
import json

servo_kit = ServoKit(channels=16)
angle = 0
servo_kit.servo[0].angle = angle
kit = MotorKit()

localIP     = "0.0.0.0"
localPort   = 14043
bufferSize  = 32768

# Bind to address and ip
UDPServerSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)
UDPServerSocket.bind((localIP, localPort))

message = None

while True:

    bytesAddressPair = UDPServerSocket.recvfrom(bufferSize)
    message = bytesAddressPair[0]    
    binData = lz4f.decompress(message, return_bytearray=True, return_bytes_read=False) 
    utfData = binData.decode('utf-8') 
    data = json.loads(utfData) 
    indexRotation = data["scene"]["actors"][0]["body"]['leftIndexMedial']['rotation']
    middleRotation = data["scene"]["actors"][0]["body"]['leftMiddleMedial']['rotation']

    #print(indexRotation["y"])

    if (indexRotation["y"] > 0.5):
       kit.motor1.throttle = 1.0
       print("Boom")

       servo_kit.servo[0].angle = angle
       angle = angle + 90 
       if angle >= 180:
          angle = 0
    else:
       kit.motor1.throttle = 0.0

    if (middleRotation["y"]  > 0.5):
       kit.motor2.throttle = 1.0
       print("Bam")

       servo_kit.servo[0].angle = angle
       angle = angle + 90 
       if angle >= 180:
          angle = 0
    else:
       kit.motor2.throttle = 0.0
#    kit.motor2.throttle = 0.0
#    sleep(1.0)
#    kit.motor1.throttle = 0.0
#    kit.motor2.throttle = 1.0


#    sleep(1.0)

