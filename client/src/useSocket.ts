import { useCallback, useEffect, useRef, useState } from "react";

const SOCKET_RECONNECT_INTERVAL = 1000;

export enum Status {
  Init = "initializing",
  Open = "open",
  Closed = "closed",
}

type OnMessageHandler = (event: MessageEvent<any>) => void;

export const useSocket = (url: string | undefined, onMessage: OnMessageHandler) => {
  const ws = useRef<WebSocket>();
  const [status, setStatus] = useState(Status.Closed)

  const onOpen = useCallback(() => {
    console.log("Connection established")
    setStatus(Status.Open)
  }, [])

  const onClose = useCallback(() => {
    console.warn("Connection closed")
    setStatus(Status.Closed);
    ws.current = undefined
  }, [])

  const onError = (error: unknown) => {
    console.error("Error:", error)
  }

  // Create new web socket and register event handlers.
  const initSocket = useCallback(() => {
    if (!url) {
      return;
    }

    console.log(`Creating connection at ${url}`);
    const socket = new WebSocket(url);
    socket.addEventListener("open", onOpen);
    socket.addEventListener("close", onClose);
    socket.addEventListener("message", onMessage)
    socket.addEventListener("error", onError)
    ws.current = socket;
  }, [url]);

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
      ws.current?.removeEventListener("error", onError)
    }
  }, [])

  return { status, url }
}