import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'

import { PuppetTheater } from './puppet-theater'
import { YoutubeChat } from './youtube-chat'

define('puppet-theater', PuppetTheater);
define('youtube-chat', YoutubeChat);

const PuppetRoot =  {
    phase: connect(store, (state) => state.phase),
    render: render(() => 
        html`
		<style>
			:host {
				display: inline-block;
				width: 100%;
				height: 100%;
			}
      #puppet-container {
        position: relative;
        height: 100%;
      }
      youtube-chat {
        position: absolute;
        top: 0;
        z-index: 1;
      }
      puppet-theater {
        position: absolute;
        top: 0;
        width: 100%;
        height: 100%;
      }
		</style>
      <div id="puppet-container">
        <puppet-theater></puppet-theater>
      </div>
     `, {shadowRoot :true})
}

export { PuppetRoot }
