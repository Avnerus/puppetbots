import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'

import { PuppetAvatar } from './puppet-avatar'

define('puppet-avatar', PuppetAvatar)

const PuppetTheater =  {
    render: render(() => 
        html`
		<style>
			#theater-background {
				width: 100%;
				height: 100%;
        background-image: url(assets/desert.jpg);
			}
		</style>
      <div id="theater-background">
        <puppet-avatar identity="1"></puppet-avatar>
        <puppet-avatar identity="2"></puppet-avatar>
      </div>
     `)
}

export { PuppetTheater }
