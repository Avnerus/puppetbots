import { useState } from 'react'
import reactLogo from './assets/react.svg'
import ReactJson from 'react-json-view'
import styled from 'styled-components'
import { useConfigEditorStore } from './store'

const Container = styled.div`
  padding: 20px;
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
`

const ConfigBox = styled.div`
  width: 800px;
  height: 200px;
  margin-bottom: 20px;
`
const JsonBox = styled.div`
  width: 800px;
  max-height: 700px;
  margin-bottom: 20px;
`
const JsonText = styled.textarea`
 width: 95%;
 height: 100%;
`
const ConfigRow = styled.div`
  display: flex;
  flex-direction: row;
`

const ButtonsRow = styled.div`
  display: flex;
  flex-direction: row;
`

const PressureBox = styled.div`
  display: flex;
  flex-direction: column;
`


function App() {
  
  const config = useConfigEditorStore((state) => state.config);
  const json = useConfigEditorStore((state) => state.json);
  const socket = useConfigEditorStore((state) => state.socket);
  const pressures = useConfigEditorStore((state) => state.pressures);


  const loadConfig= () => {
    const command = 'CGET';
    const buffer = new ArrayBuffer(command.length);
    const z = new Uint8Array(buffer);
    let pos = 0;
    for (let i = 0; i < command.length; i++) {
        z[pos] = command.charCodeAt(i);
        pos++;
    }
    socket.send(buffer);
  }

  const onEdit = (e) => {
    console.log("Config edit", e);
  }

  const jsonChanged = (e) => {

  }

  return (
    <Container>
      <h1>Puppet controller config editor</h1>
      <ConfigRow>
        <JsonBox>
          <JsonText value={json} onChange={jsonChanged}></JsonText>
        </JsonBox>
        <ConfigBox>
          <ReactJson
            src={config}
            theme={'flat'}
            onEdit={onEdit}
          />
        </ConfigBox>
      </ConfigRow>
      <ButtonsRow>
          <button onClick={loadConfig}>Load from device</button>
      </ButtonsRow>
      <PressureBox>
        <h2>Pressure values</h2>
        <ul>
        {Object.entries(pressures).map(([key, value]) => <li key={key}>{key}: {value}</li>)}
        </ul>
      </PressureBox>
    </Container>
  )
}

export default App
