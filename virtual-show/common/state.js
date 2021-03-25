import {createStore} from 'redux';

import SocketController from '../common/socket-controller'

export const ROLES = {
    CONTROLLER: "CONTROL",
    AVATAR: "AVATAR"
}

export const OTHER = {
    "CONTROL": "AVATAR",
    "AVATAR": "CONTROL"
}

const reducer = (state = {
    identity: null,
    socketController: null,
    keyboard:  null,
    listener: null,
    puppetState: {
    },
}, action) => {
  switch (action.type) {
    case 'SET_SOCKET_CONTROLLER': {
        if (action.manage) {
            action.socketController.subscribeToPrefix('E', (msg) => {
                console.warn(msg.slice(1));
            });
            action.socketController.subscribeToPrefix('I', (msg) => {
                store.dispatch(addTranscript({
                    from: "System",
                    text: msg.slice(1)
                }));
            });
        }
        action.socketController.on('puppet-state', (data) => {
            console.log("New puppet state!", data.state);
            store.dispatch(setPuppetState(data.state));
        });
        return {...state, socketController: action.socketController}
    }
    case 'SET_PUPPET_STATE' : {
        return {...state, puppetState : action.value}
    }
    case 'SET_KEYBOARD' : {
        console.log("Set keyboard", action.value);
        const keyboard = action.value; 
        keyboard.onPress(39, () => {
          store.dispatch(keyPress(39));
        });
        return {...state, keyboard}
    }
    case 'KEY_PRESS': {
      const keyCode = action.value
      const puppetState = {...state.puppetState};
      if (keyCode == 39) {
        console.log("Right!", state.identity, state.puppetState)
        if (state.identity && puppetState[state.identity]) {
          puppetState[state.identity].position[0] += 1;
        }
      }
      return {...state, puppetState}
    }
    case 'SET_LISTENER' : {
        console.log("Set listener", action.value);
        return {...state, listener : action.value}
    }
    case 'SET_IDENTITY' : {
        return {...state, identity : action.value}
    }
    default:
      return state;
  };
}

// Store instance as default export
const store = createStore(
  reducer,
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

export const connect = (store, mapState) => ({
  get: mapState ? () => mapState(store.getState()) : () => store.getState(),
  connect: (host, key, invalidate) => store.subscribe(invalidate)
});
