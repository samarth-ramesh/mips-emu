import { useEffect, useState } from "react"
import init, { greet } from "mips_be"
import TextBox from "./app/TextBox"

function App() {
  const [available, setAvailable] = useState(false)
  useEffect(() => {
    init().then(_ => {
      setAvailable(true)
    })
  })

  if (available) {
    return (
        <div className="flex flex-col items-center justify-evenly min-h-screen">
          <h1 className="text-5xl">MIPS Emulator</h1>
          <TextBox />
        </div>
      )
  } else {
    return <div>Loading...</div>
  }
}


export default App
