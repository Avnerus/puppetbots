import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'

import { PuppetTheater } from './puppet-theater'
import { YoutubeChat } from './youtube-chat'
import { PuppetAction } from './puppet-action'

define('puppet-theater', PuppetTheater);
define('puppet-action', PuppetAction);
define('youtube-chat', YoutubeChat);

const PuppetRoot =  {
    identity: connect(store, (state) => state.identity),
    render: render(({ identity }) => 
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
        display: flex;
        justify-content: center;
        align-items: center;
      }
      puppet-theater {
        width: 80%;
        height: 94%;
      }
		</style>
      <div id="puppet-container">
        <youtube-chat></youtube-chat>
        ${identity && html`<puppet-action identity="${identity}"></puppet-action>`}
        <puppet-theater></puppet-theater>
      </div>
     `, {shadowRoot :true})
}

export { PuppetRoot }
