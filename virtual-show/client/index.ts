import '@webcomponents/webcomponentsjs/webcomponents-bundle.js';

import store, {setSocketController, setListener, setKeyboard, setIdentity} from '../common/state'

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

//const socketController = new SocketController(`ws://${window.location.hostname}:3012`,() => {
const socketController = new SocketController(`ws://2.tcp.ngrok.io:17264`,() => {
  store.dispatch(setSocketController(socketController, true))
  if (puppeteer) {
    store.dispatch(setIdentity(Number(puppeteer)));
    socketController.send("R" + String.fromCharCode(Number(puppeteer)) + "Puppeteer " + puppeteer);
    keyboard.grab();
    store.dispatch(setKeyboard(keyboard));
  }
});

socketController.init();

if (module.hot) {
    console.log("We have hot");
    module.hot.accept('./puppet-root.ts', function() {
      define('puppet-layout', PuppetLayout);
    })
}

