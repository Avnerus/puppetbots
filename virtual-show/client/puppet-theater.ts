import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'

import { PuppetAvatar } from './puppet-avatar'
import { TheaterVoice } from './theater-voice'

define('puppet-avatar', PuppetAvatar)
define('theater-voice', TheaterVoice)

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
      <theater-voice
          roomId="${9540}"
          streamURL="${'https://tdialogos.aalto.fi/janus'}"
        >
     `)
}

export { PuppetTheater }
