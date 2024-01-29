import { useState } from 'react';
import { useSocket } from './useSocket';

import './App.css'

const SOCKET_URL = "ws://127.0.0.1:8080/ws"

function App() {
  const [url, setUrl] = useState<string>();

  const { status } = useSocket(SOCKET_URL, (event: MessageEvent<any>) => {
    console.debug("Message received on socket", event)
    handleImageBlob(event.data)
  });

  // Create PNG blob from bytes received.
  const handleImageBlob = async (blob: Blob) => {
    const buffer = await blob.arrayBuffer();
    const pngBlob = new Blob([buffer], { type: "image/png" });
    const url = URL.createObjectURL(pngBlob)
    setUrl(url)
  }

  return (
    <>
      <div>
        <div>
          Connection: {status}
        </div>
        <img src={url} width={200} />
      </div>
    </>
  )
}

export default App
