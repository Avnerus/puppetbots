import {createStore} from 'redux';

import SocketController from '../common/socket-controller'

enum Direction {
  Left = 37,
  Up = 38 ,
  Right = 39,
  Down = 40,
}

const reducer = (state = {
    identity: null,
    socketController: null,
    keyboard:  null,
    listener: null,
    audioStream: null,
    chats: [],
    puppetState: {
    },
}, action) => {
  switch (action.type) {
    case 'SET_SOCKET_CONTROLLER': {
        if (action.manage) {
            action.socketController.subscribeToPrefix('E', (msg) => {
                console.warn(msg.slice(1));
            });
        }
        action.socketController.on('puppet-state', (data) => {
            console.log("New puppet state!", data.state);
            store.dispatch(setPuppetState(data.state));
        });
        action.socketController.on('youtube-chat', (data) => {
            console.log("Youtube chat!!", data);
            store.dispatch(addChats(data.messages));
        });
        return {...state, socketController: action.socketController}
    }
    case 'SET_PUPPET_STATE' : {
        return {...state, puppetState : action.value}
    }
    case 'SET_KEYBOARD' : {
        console.log("Set keyboard", action.value);
        const keyboard = action.value; 
        for (const direction of [Direction.Left, Direction.Up, Direction.Right, Direction.Down]) {
          keyboard.onPress(direction, () => {
            store.dispatch(keyPress(direction));
          });
        };
        return {...state, keyboard}
    }
    case 'KEY_PRESS': {
      const keyCode = action.value
      const puppetState = {...state.puppetState};
      if (state.identity && puppetState[state.identity]) {
        switch (keyCode) {
          case Direction.Up:
            puppetState[state.identity].position[1] += 1;
            break;
          case Direction.Down:
            puppetState[state.identity].position[1] -= 1;
            break;
          case Direction.Right:
            puppetState[state.identity].position[0] += 1;
            break;
          case Direction.Left:
            puppetState[state.identity].position[0] -= 1;
            break;
        }
        state.socketController.sendValueCommand(
          "SPOS",
          puppetState[state.identity].position[0],
          puppetState[state.identity].position[1]
        );
      }
      return {...state, puppetState}
    }
    case 'SET_LISTENER' : {
        return {...state, listener : action.value}
    }
    case 'SET_IDENTITY' : {
        return {...state, identity : action.value}
    }
    case 'SET_AUDIO_STREAM' : {
        return {...state, audioStream : action.value}
    }
    case 'SET_ACTION' : {
      const puppetState = {...state.puppetState};
      if (state.identity && puppetState[state.identity]) {
        puppetState[state.identity].action = action.value;
        state.socketController.sendValueCommand(
          "SACT",
          puppetState[state.identity].action
        );
      }
      return {...state}
    }
    case 'ADD_CHATS' : {
      return {...state, chats: [...state.chats, ...action.value]}
    }
    default:
      return state;
  };
}

// Store instance as default export
const store = createStore(
  reducer,
  // @ts-ignore
  window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__(), // redux dev tools
);

export default store;

export const setSocketController = (socketController, manage) => ({
    type: 'SET_SOCKET_CONTROLLER',
    socketController,
    manage
})

export const setPuppetState = (value) => ({
    type: 'SET_PUPPET_STATE',
    value,
})


export const setListener = (value) => ({
    type: 'SET_LISTENER',
    value
})

export const setKeyboard = (value) => ({
    type: 'SET_KEYBOARD',
    value
}) 

export const keyPress = (value) => ({
  type: 'KEY_PRESS',
  value
})

export const setIdentity = (value) => ({
    type: 'SET_IDENTITY',
    value
})

export const setAudioStream = (value) => ({
    type: 'SET_AUDIO_STREAM',
    value
})

export const setAction = (value) => ({
    type: 'SET_ACTION',
    value
})

export const addChats = (value) => ({
    type: 'ADD_CHATS',
    value
})

export const connect = (store, mapState) => ({
  get: mapState ? () => mapState(store.getState()) : () => store.getState(),
  connect: (host, key, invalidate) => store.subscribe(invalidate)
});
