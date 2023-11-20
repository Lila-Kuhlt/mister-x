import { Map } from "components/Map";
import { BASE_URLS, ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockets";
import { useEffect, useState } from "react";

export function Game() {
  const [ws, setWS] = useState<WebsocketApi | undefined>();
  useEffect(() => {
    if (!ws) return;

    const socket = new WebsocketApi(
      BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS,
      setWS
    ).register("Position", console.log);

    return () => socket.disconnect();
  }, []);
  return <Map></Map>;
}
