import { useState, useEffect } from "preact/hooks"
import preactLogo from "./assets/preact.svg"
import { invoke } from "@tauri-apps/api/tauri"
import "./App.css"
import { Device } from './Device'

function App() {
  const [devices, setDevices] = useState([])
  const [name, setName] = useState("")

  useEffect(() => {
    invoke("get_devices").then(setDevices)
  }, [])

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://preactjs.com" target="_blank">
          <img src={preactLogo} class="logo preact" alt="Preact logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Preact logos to learn more.</p>

      <Device/>
    </div>
  )
}

export default App
