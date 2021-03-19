import { html, render } from 'hybrids';
import store, {connect} from '../common/state'

const PuppetAvatar =  {
    puppetState: connect(store, (state) => state.puppetState),
    identity: "",
    render: render(({puppetState, identity}) => 
        html`
		<style>
      #avatar {
        position: absolute;
        bottom: 80px;
        height: 250px;
        display: flex;
      }
		</style>
      <div id="avatar">
        <img src="assets/avatar-${identity}.png">
      </div>
     `)
}

export { PuppetAvatar }
