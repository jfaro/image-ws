import { useCallback, useEffect, useRef, useState } from 'react';
import './App.css'

const SOCKET_URL = "ws://127.0.0.1:8080/ws"
const SOCKET_RECONNECT_INTERVAL = 1000;

enum Status {
  Init = "initializing",
  Open = "open",
  Closed = "closed",
}

const useSocket = () => {
  const ws = useRef<WebSocket>();
  const [status, setStatus] = useState(Status.Closed)
  const [message, setMessage] = useState("");

  // Handle socket open.
  const onOpen = useCallback(() => {
    console.log("Connection established")
    setStatus(Status.Open)
  }, [])

  // Handle socket close.
  const onClose = useCallback(() => {
    console.warn("Connection closed")
    setStatus(Status.Closed);
    ws.current = undefined
  }, [])

  // Handle socket message.
  const onMessage = (event: MessageEvent<any>) => {
    console.log("Message received on socket", event.data)
    setMessage(event.data)
  };

  // Create new web socket and register event handlers.
  const initSocket = useCallback(() => {
    console.log(`Creating connection at ${SOCKET_URL}`);
    const socket = new WebSocket(SOCKET_URL);
    socket.addEventListener("open", onOpen);
    socket.addEventListener("close", onClose);
    socket.addEventListener("message", onMessage)
    socket.addEventListener("error", (error) => {
      console.error("Error:", error)
    })
    ws.current = socket;
  }, []);

  useEffect(() => {
    let interval = setInterval(() => {
      // Clear closed connection.
      if (ws.current && ws.current.readyState === ws.current.CLOSED) {
        console.warn("Removing closed connection")
        ws.current = undefined;
      }

      // Initialize connection.
      if (!ws.current) {
        initSocket()
      }
    }, SOCKET_RECONNECT_INTERVAL)

    return () => {
      clearInterval(interval)
      ws.current?.removeEventListener("open", onOpen)
      ws.current?.removeEventListener("close", onClose)
      ws.current?.removeEventListener("message", onMessage)
    }
  })

  return { status, message }
}

function App() {
  const { status, message } = useSocket();

  return (
    <>
      <div>
        <div>
          Connection: {status}
        </div>
        <div>
          Message: {message}
        </div>
      </div>
    </>
  )
}

export default App
