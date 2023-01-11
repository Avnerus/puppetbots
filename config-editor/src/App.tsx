import { useState } from 'react'
import reactLogo from './assets/react.svg'
import ReactJson from 'react-json-view'
import styled from 'styled-components'

const Container = styled.div`
  padding: 20px;
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
`

const JsonBox = styled.div`
  width: 800px;
  max-height: 800px;
`

const ButtonsRow = styled.div`
  display: flex;
  flex-direction: row;
`

function App() {
  
  const [config, setConfig] = useState({})

  return (
    <Container>
      <JsonBox>
        <ReactJson
          src={config}
          theme={'flat'}
        />
      </JsonBox>
      <ButtonsRow>
          <button>Load from device</button>
      </ButtonsRow>
    </Container>
  )
}

export default App
