import React, { useEffect } from 'react';
import { Observer } from 'mobx-react'
import { Video } from './components/Video';
import { useEngines } from './stores/use_engines';

function App() {
  const { wasmEngine } = useEngines()

  useEffect(() => {
    console.log("SUP")
    async function loadWasm() {
        await wasmEngine.initialize()
    }
    loadWasm()
  }, [])

  return (
    <Observer>
      {() => wasmEngine.loading ? <h1>Loading...</h1> : <Video />}
    </Observer>
  )

}

export default App;
