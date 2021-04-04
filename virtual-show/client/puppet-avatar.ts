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
        left: ${puppetState[identity] ? puppetState[identity].position[0] - 100 : 0}%;
        bottom: ${puppetState[identity] ? puppetState[identity].position[1] - 100 : 0}%;
        height: 250px;
        display: ${(puppetState[identity] && puppetState[identity].connected) ? 'flex' : 'none'};
      }
		</style>
      <div id="avatar">
        <img src="assets/avatar-${identity}.png">
      </div>
     `)
}

export { PuppetAvatar }
