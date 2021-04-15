import { html, render, define } from 'hybrids';
import store, { connect, setAudioStream } from '../common/state'
import { PuppetAvatar } from './puppet-avatar'

// Beacause of using webpack exports-loader
// @ts-ignore
const YoutubeChat =  {
    id: "",
    render: ({ id }) => 
        html`
          <style>
            :host {
              width: 100%;
              display: block;
            }
            #item-scroller {
              overflow-y: hidden !important;
            }
          </style>
          <iframe 
              width=100%
              height=200 
              src="https://www.youtube.com/live_chat?v=${id}&embed_domain=${window.location.hostname}"
          >
          </iframe>
     `
}

export { YoutubeChat }
