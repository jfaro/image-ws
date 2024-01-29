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
  // const [ws, setWs] = useState<WebSocket>();
  const [status, setStatus] = useState(Status.Closed)

  // Handle socket open.
  const onOpen = useCallback(() => {
    console.log("Connection established")
    setStatus(Status.Open)
  }, [])

  // Handle socket close.
  const onClose = useCallback(() => {
    console.log("Connection closed")
    setStatus(Status.Closed);
    ws.current = undefined
  }, [])

  // Handle socket message.
  const onMessage = useCallback((event: MessageEvent<any>) => {
    console.log("Message received on socket", event.data)
  }, []);

  // Create new web socket and register event handlers.
  const initSocket = useCallback(() => {
    console.log(`Creating connection at ${SOCKET_URL}`)
    const socket = new WebSocket(SOCKET_URL);
    socket.addEventListener("open", onOpen);
    socket.addEventListener("close", onClose);
    socket.addEventListener("message", onMessage)
    ws.current = socket;
  }, []);

  useEffect(() => {
    let interval = setInterval(() => {
      // Clear closed connection.
      if (ws.current && ws.current.readyState === ws.current.CLOSED) {
        console.log("Removing closed connection")
        ws.current = undefined;
      }

      // Initialize connection.
      if (!ws.current) {
        initSocket()
      }
    }, SOCKET_RECONNECT_INTERVAL)

    return () => {
      clearInterval(interval)
      ws.current?.removeEventListener("open", (_) => onOpen())
      ws.current?.removeEventListener("close", onClose)
      ws.current?.removeEventListener("message", onMessage)
    }
  })

  return { status }
}

function App() {
  const { status } = useSocket();

  return (
    <>
      <div>
        Connection: {status}
      </div>
    </>
  )
}

export default App
