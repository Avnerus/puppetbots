import { html, render, define } from 'hybrids';
import store, { connect, setAudioStream, clearChats } from '../common/state'
import { PuppetAvatar } from './puppet-avatar'

const arrangeChats = (host, event) => {
  const messages = host.querySelectorAll('.chat-message')
  let delay = 0;

  for (const message of messages) {
    const flagged = message.dataset.flagged;
    const uuid = message.dataset.uuid;
    const style = message.style;

    if (flagged == 'false') {
      message.dataset.flagged = 'true';
      setTimeout(() => {
        message.classList.add('fade');
      },delay)

      setTimeout(() => {
        style.display = "none";
        if (allClear(host)) {
          store.dispatch(clearChats());
        }
      },delay + 12000)

      delay += 1000;
    }
  }
}

const allClear = (host) => {
  const messages = host.querySelectorAll('.chat-message')

  for (const message of messages) {
    if (message.style.display != 'none') {
      return false;
    }
  }
  return true;
}

const chatArranger = {
    render: () => html`
        <style>
            :host {
               position: absolute;
              display: flex;
              flex-flow: wrap;
              flex-direction: column;
              height: 100%;
              overflow: hidden;
            } 
        </style>
        <slot onslotchange="${arrangeChats}"></slot>
    `
}

define('chat-arranger', chatArranger)

const YoutubeChat =  {
    chats: connect(store, (state) => state.chats),
    render: ({ chats }) => 
        html`
          <style>
            :host {
              width: 80%;
              position: absolute;
              top: 5%;
              height: 40%;
            }
            .chat-message {
              display: flex;
              max-width: 40%;
              min-width: 300px;
              background-color: #151515bf;
              margin: 10px;
              min-height: 15%;
              align-items: center;
              color: white;
              border-width: 2px;
              border-color: #27bfb1;
              border-style: solid;
              border-radius: 20px;
              font-size: 20px;
              opacity: 0;
            }
            .chat-text {
              color: white;
              margin: 10px;
              word-wrap: anywhere;
            }
            .chat-author {
              font-weight: bold;
              color: #48ff2e;
              margin-left: 10px;
              margin-right: 10px;
            }

            .chat-message.fade {
                animation:
                  fade-in-out 12s forwards;
            }

            @keyframes fade-in-out {
                0% { opacity: 0; }
                25% { opacity: 1; }
                60% { opacity: 1; }
                100% { opacity: 0; }
            }

          </style>

          <chat-arranger>
              ${chats.map(( message ) => html
                  `<div class="chat-message" data-uuid="${message.uuid}" data-flagged="false"><span class="chat-author">${message.author}:</span><span class="chat-text">${message.text}</span></div>
              `)}
          </chat-arranger>
     `
}

export { YoutubeChat }
