import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'
import ReconnectingWebSocket from 'reconnecting-websocket'

const socket = new ReconnectingWebSocket('ws://localhost:3012');
socket.binaryType = "arraybuffer";

interface ConfigEditorState {
  socket?: WebSocket
  config: any
}

const useConfigEditorStore = create<ConfigEditorState>()(
  devtools(
    persist(
      (set) => ({
        socket: undefined,
        config: { empty: 1 },
        json: '{"empty": 1"}',
        setConfig: (config) => set((state) => ({ config })),
        setJson: (json) => set((state) => ({ json })),
        setSocket: (socket) => set((state) => ({ socket })),
      }),
      {
        name: 'config-editor-storage',
      }
    )
  )
)



function sendValueCommand(command, ...values) {
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
    socket.send(buffer);
}

socket.addEventListener('open', () => {
  // Register as admin
  sendValueCommand('R', 0);
  useConfigEditorStore.getState().setSocket(socket);
})

socket.addEventListener('message', (msg) => {
  console.log("Message!", msg.data);
  if (msg.data instanceof ArrayBuffer) {
      const prefix = String.fromCharCode(new Uint8Array(msg.data,0,1)[0]);
      console.log("Array buffer prefix", prefix);

      if (prefix == 'F') {
          // Parse it
          const chars = new Uint8Array(msg.data, 1);
          const json = new TextDecoder("utf-8").decode(chars);
          const config = JSON.parse(json);
          useConfigEditorStore.getState().setConfig(config);
          useConfigEditorStore.getState().setJson(json);
      }
  }
})

export { useConfigEditorStore }

