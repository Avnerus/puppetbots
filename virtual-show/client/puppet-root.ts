import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'

import { PuppetTheater } from './puppet-theater'

define('puppet-theater', PuppetTheater);

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
		</style>
      <puppet-theater></puppet-theater>
     `, {shadowRoot :true})
}

export { PuppetRoot }
