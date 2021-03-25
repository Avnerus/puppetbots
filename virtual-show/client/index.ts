import '@webcomponents/webcomponentsjs/webcomponents-bundle.js';

import store, {setSocketController, setListener, setKeyboard, setIdentity, ROLES} from '../common/state'

import { define } from 'hybrids';
import { PuppetLayout } from './puppet-layout'
import { PuppetRoot } from './puppet-root'

import SocketController from '../common/socket-controller'
import { Keyboard } from '../common/keyboard'

define('puppet-layout', PuppetLayout);
define('puppet-root', PuppetRoot);

console.log("Loading client");

const urlParams = new URLSearchParams(window.location.search);
const puppeteer = urlParams.get('puppeteer');

const keyboard = new Keyboard();

const socketController = new SocketController('ws://127.0.0.1:3012',() => {
  store.dispatch(setSocketController(socketController, true))
  if (puppeteer) {
    store.dispatch(setIdentity(Number(puppeteer)));
    socketController.send("R" + String.fromCharCode(Number(puppeteer)) + "Puppeteer " + puppeteer);
    keyboard.grab();
    store.dispatch(setKeyboard(keyboard));
  }
});

socketController.init();

store.dispatch(setIdentity(ROLES.CONTROLLER));

if (module.hot) {
    console.log("We have hot");
    module.hot.accept('./puppet-root.ts', function() {
      define('puppet-layout', PuppetLayout);
    })
}

