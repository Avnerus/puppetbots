import socket
import lz4.frame as lz4f
import json

localIP     = "0.0.0.0"
localPort   = 14043
bufferSize  = 32768

# Bind to address and ip
UDPServerSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)
UDPServerSocket.bind((localIP, localPort))

print("UDP server up and listening")

message = None

breakFlag = False

while(not breakFlag):

    bytesAddressPair = UDPServerSocket.recvfrom(bufferSize)
    message = bytesAddressPair[0]    
    binData = lz4f.decompress(message, return_bytearray=True, return_bytes_read=False) 
    utfData = binData.decode('utf-8') 
    data = json.loads(utfData) 
    print(data)
    breakFlag = True
   