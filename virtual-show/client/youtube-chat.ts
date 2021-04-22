import { html, render, define } from 'hybrids';
import store, { connect, setAudioStream } from '../common/state'
import { PuppetAvatar } from './puppet-avatar'

const arrangeChats = (host, event) => {
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
              width: 100%;
              position: absolute;
              top: 0;
              height: 40%;
            }
            .chat-message {
              display: flex;
              max-width: 30%;
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
            }
            .chat-text {
              color: white;
              margin: 10px;
            }
            .chat-author {
              font-weight: bold;
              color: #48ff2e;
              margin-left: 10px;
              margin-right: 10px;
            }
          </style>

          <chat-arranger>
              ${chats.map(( message ) => html
                  `<div class="chat-message"><span class="chat-author">${message.author}:</span><span class="chat-text">${message.text}</span></div>
              `)}
          </chat-arranger>
     `
}

export { YoutubeChat }
