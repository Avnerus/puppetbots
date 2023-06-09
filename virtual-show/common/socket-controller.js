// For real time communication
import WebSocket from 'reconnecting-websocket'

export default class SocketController {
    constructor(host, connectCallback = null) {
        this.host = host;
        this.prefixes = {};
        this.commands = {};
        this.connectCallback = connectCallback;
    }
    init() {
        //let host = document.location.host;
        this.socket = new WebSocket(this.host);
        this.socket.binaryType = "arraybuffer";
        this.socket.addEventListener('open', () => {this.onConnect()});
        this.socket.addEventListener('message', (msg) => {this.onMessage(msg)});
    }
    onConnect() {
        console.log("Socket connected!");
        if (typeof(events) != 'undefined') {
            events.emit("socket_connected", this.socket);
        }
        if (this.connectCallback) {
            this.connectCallback();
        }
    }
    emit(message, args) {
        if (this.socket) {
            this.socket.emit(message, args);
        }
    }

    send(message) {
        if (this.socket) {
            this.socket.send(message);
        }
    }

    onMessage(msg) {
        // console.log("Socket controller message: ", msg);
        let prefix = '';

        if (msg.data instanceof ArrayBuffer) {
            prefix = String.fromCharCode(new Uint8Array(msg.data,0,1)[0]);
            //console.log("Array buffer prefix", prefix);

            if (prefix == 'C') {
                // Parse it
                let chars = new Uint16Array(msg.data, 2);
                let json = new TextDecoder("utf-16").decode(chars);
                let obj = JSON.parse(json);
                if (this.commands[obj.command]) {
                    this.commands[obj.command](obj);
                }
            }
            if (prefix == 'U') {
                // Parse it
                let chars = new Uint8Array(msg.data, 1);
                let json = new TextDecoder("utf-8").decode(chars);
                let obj = JSON.parse(json);
                if (this.commands[obj.command]) {
                    this.commands[obj.command](obj);
                }
            }
        }
        else {
            prefix = msg.data[0];
        }
        if (this.prefixes[prefix]) {
            this.prefixes[prefix](msg.data);
        }
    }

    subscribeToPrefix(prefix, callback) {
        this.prefixes[prefix] = callback;        
    }

    sendSerialCommand(command, ...values) {
        let buffer = new ArrayBuffer(3 + values.length);
        let z = new Uint8Array(buffer);
        z[0] = ">".charCodeAt(0);
        z[1] = command.charCodeAt(0); 
        z[2] = values.length;
        for (let i = 0; i < values.length; i++) {
            z[i + 3] = values[i];
        }
        this.send(buffer);
    }
    sendValueCommand(command, ...values) {
        let buffer = new ArrayBuffer(command.length + values.length);
        let z = new Uint8Array(buffer);
        let pos = 0;
        for (let i = 0; i < command.length; i++) {
            z[pos] = command.charCodeAt(i);
            pos++;
        }
        for (let i = 0; i < values.length; i++) {
            z[pos] = values[i];
            pos++;
        }
        this.send(buffer);
    }

    sendJSONCommand(obj) {
        let text = JSON.stringify(obj);
        let buffer = new ArrayBuffer(text.length * 2 + 2);
        let command = new Uint8Array(buffer, 0);
        command[0] = 'C'.charCodeAt(0); 
        let bufView = new Uint16Array(buffer, 2);
        for (let i = 0; i < text.length; i++) {
            bufView[i] = text.charCodeAt(i);
        }
        this.send(buffer);
    }

    on(command, func) {
        this.commands[command] = func;
    }
    off(command) {
        if (this.commands[command]) {
            delete this.commands[command];
        }
    }
}
