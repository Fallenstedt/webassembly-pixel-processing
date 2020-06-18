import React, { useState, useEffect } from 'react';
import { Video } from './components/Video';

function App() {
  const [loading, setLoading] = useState(true);
  const [wasm, setWasm] = useState<any>(null);

  useEffect(() => {
    async function loadWasm() {
      try {
        setLoading(true)
        const wasm = await import("converter");
        console.log(wasm)
        wasm.init()
        wasm.greet()
        setWasm(wasm)
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
