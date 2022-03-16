import socket
import lz4.frame as lz4f

localIP     = "127.0.0.1"
localPort   = 14043
bufferSize  = 32768

# Bind to address and ip
UDPServerSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)
UDPServerSocket.bind((localIP, localPort))

print("UDP server up and listening")

message = None

while(True and not message):

    bytesAddressPair = UDPServerSocket.recvfrom(bufferSize)
    message = bytesAddressPair[0]    
    dataJSON = lz4f.decompress(message, return_bytearray=True, return_bytes_read=False)     
    print(dataJSON)
   