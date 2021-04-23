import { html, render } from 'hybrids';
import store, {connect} from '../common/state'

const getImage = (identity, puppetState) => {
  return `${(identity)}${(puppetState[identity] && puppetState[identity].action) ? '-action' : ''}`
}

const getItemImage = (identity, puppetState) => {
  return `${identity}-item`
}

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
      #avatar-item {
        display: flex;
      }
      .item-1 {
        height: 150px;
        position: relative;
        right: 170px;
        top: 65px;
      }
      .item-1-action {
        height: 150px;
        position: relative;
        right: 270px;
        top: 220px;
      }
      .item-2 {
        height: 110px;
        position: relative;
        right: 185px;
        bottom: 63px;
      }
      .item-2-action {
        height: 110px;
        position: relative;
        right: 185px;
        bottom: 63px;
      }
		</style>
      <div id="avatar">
        <img src="assets/avatar-${getImage(identity, puppetState)}.png">
        <div id="avatar-item" class="item-${getImage(identity, puppetState)}">
          <img src="assets/avatar-${getItemImage(identity, puppetState)}.png">
        </div>
      </div>
     `)
}

export { PuppetAvatar }
