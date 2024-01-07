import './App.css'
import { useEffect, useRef } from 'react';

const URL = "ws://127.0.0.1:8080"

function App() {
  const connection = useRef<WebSocket | null>(null);

  useEffect(() => {
    const socket = new WebSocket(URL);

    // Connection opened.
    socket.addEventListener("open", (event) => {
      console.log("New connection opened", event)
      socket.send("Connection established")
    })

    // Listen for messages.
    socket.addEventListener("message", (event) => {
      console.log("Message from server", event.data)
    })

    // Save ref to connection.
    connection.current = socket;

    // Close connection on unmount.
    return () => socket.close()
  }, [])

  return (
    <>
      <div>
      </div>
    </>
  )
}

export default App
