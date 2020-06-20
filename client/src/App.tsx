import React, { useState, useEffect } from 'react';
import { Video } from './components/Video';
import { useEngines } from './stores/use_engines';

function App() {
  const [loading, setLoading] = useState(true);
  const {wasmEngine } = useEngines()

  useEffect(() => {
    async function loadWasm() {
      try {
        setLoading(true)
        await wasmEngine.initialize()
        wasmEngine.instance?.greet()
        console.log(wasmEngine.instance)
      } finally {
        setLoading(false)
      }
    }
    loadWasm()
  }, [])

  return (
    <div>
      {loading ? <h1>Loading...</h1> : <Video />}
    </div>
  )

}

export default App;
