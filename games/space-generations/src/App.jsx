import { useWasm } from './hooks/useWasm'
import { getMetaInfo } from './wasm/echo_frame_engine'

function App() {
  const [info, isLoading] = useWasm(() => getMetaInfo())

  if (isLoading) {
    return (
      <>
        <h1>Loading...</h1>
      </>
    )
  }

  return (
    <>
      <h1>Space Generations</h1>
      <p>Core Version: {info.version}</p>
    </>
  )
}

export default App
