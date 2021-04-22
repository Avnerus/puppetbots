import { html, render } from 'hybrids';
import store, { connect, setAction} from '../common/state'

const action = (host) => {
  store.dispatch(setAction(!(host.puppetState[host.identity].action)));
}

const PuppetAction =  {
    puppetState: connect(store, (state) => state.puppetState),
    identity: 0,
    render: render(({puppetState, identity}) => 
        html`
		<style>
      :host {
        position: absolute;
        ${identity == 1 ? 'left' : 'right'}: 3vw;
        bottom: 50vh;
      }
      input {
        height: 80px;
        width: 80px;
        z-index: 1;
        font-size: 20px;
      }
		</style>
      <div id="puppet-action">
        <input type="button" value="Action" onclick=${action}>
      </div>
     `)
}

export { PuppetAction }
