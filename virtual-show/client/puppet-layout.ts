import { html, render } from 'hybrids';

const PuppetLayout = {
    render: ({state}) => html`

        <style>
            :host {
                width: 100vw;
                height: 100vh;
                position: absolute;
                padding: 0;
                margin: 0;
                top: 0;

                display: flex;
                align-items: center;
                justify-content: center;

                background-color: #fdffff;

				font-family: Roboto;
            }
			::slotted(div) {
				width: 95vw;
				height: 95vh;
				background-color: #f0f0f0;
			}
        </style>
		<slot name="content">
		</slot>
     `
}

export { PuppetLayout }
