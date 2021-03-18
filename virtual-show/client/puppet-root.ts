import { html, render } from 'hybrids';
import store, {connect} from '../common/state'

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
      <h1>Hello world</h1>
     `, {shadowRoot :true})
}

export { PuppetRoot }
