import {createStore} from 'redux';
import { v4 as uuidv4 } from 'uuid';

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
    chats: [ ],
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
            store.dispatch(setPuppetState(data.state));
        });
        action.socketController.on('youtube-chat', (data) => {
            store.dispatch(addChats(data.messages));
        });
        return {...state, socketController: action.socketController}
    }
    case 'SET_PUPPET_STATE' : {
        return {...state, puppetState : action.value}
    }
    case 'SET_KEYBOARD' : {
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
      const chats = action.value
      for (const message of chats) {
        message.uuid = uuidv4();
      }
      return {...state, chats: [...state.chats, ...chats]}
    }
    case 'REMOVE_CHAT_BY_UUID' : {
      const index = state.chats.findIndex(e => e.uuid == action.value);
      return {...state, 
        chats: [
          ...state.chats.slice(0, index),
          ...state.chats.slice(index + 1)
        ]
      }
    }
    case 'CLEAR_CHATS' : {
      return {...state,  chats: [] }
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

export const removeChatByUuid = (value) => ({
    type: 'REMOVE_CHAT_BY_UUID',
    value
})

export const clearChats = () => ({
    type: 'CLEAR_CHATS'
})

export const connect = (store, mapState) => ({
  get: mapState ? () => mapState(store.getState()) : () => store.getState(),
  connect: (host, key, invalidate) => store.subscribe(invalidate)
});
