import { useState } from 'react';
import { useSocket } from './useSocket';

import './App.css'

const SOCKET_URL = "ws://127.0.0.1:8080/ws"

function App() {
  // Image URL for content to display.
  const [url, setUrl] = useState<string>();

  // Register message handler for WebSocket.
  const { status } = useSocket(SOCKET_URL, (event: MessageEvent<any>) => {
    console.debug("Message received on socket", event)
    handleImageBlob(event.data)
  });

  // Create PNG blob from bytes received.
  const handleImageBlob = async (blob: Blob) => {
    const buffer = await blob.arrayBuffer();
    const png = new Blob([buffer], { type: "image/png" });
    setUrl(URL.createObjectURL(png))
  }

  return (
    <>
      <div className="container">
        <div>
          Connection is {status}
        </div>
        <img src={url} width={200} height={200} />
      </div>
    </>
  )
}

export default App
