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
        <TextBox/>
      )
  } else {
    return <div>Loading...</div>
  }
}


export default App
