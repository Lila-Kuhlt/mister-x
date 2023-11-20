import { Map } from "components/Map";
import { BASE_URLS, ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockets";
import { createContext, useEffect, useState } from "react";

export const WebsocketContext = createContext<WebsocketApi | undefined>(
  undefined
);

export function Game() {
  const [ws, setWS] = useState<WebsocketApi | undefined>();

  useEffect(() => {
    const WS_URL = BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS;
    const socket: WebsocketApi = new WebsocketApi(WS_URL);

    socket
      .registerEvent("Connect", () => setWS(socket))
      .registerEvent("Disconnect", () => setWS(undefined))
      .registerEvent("Error", () => setTimeout(() => socket.reconnect(), 1000));

    socket.register("GameState", (gs) => console.log("GameState: ", gs));

    return () => socket.disconnect();
  }, []);

  // Loading page
  const LOADER = () => (
    <div className="flex flex-col items-center justify-center gap-5 w-max h-max">
      <div className="flex flex-col items-center">
        <span className="italic text-slate-400">
          Connection to Gameserver lost
        </span>
        <span className="italic text-slate-400">
          Attempting to reconnect...
        </span>
      </div>
    </div>
  );

  const MAP = () => (
    <WebsocketContext.Provider value={ws}>
      <Map></Map>
    </WebsocketContext.Provider>
  );

  return ws ? MAP() : LOADER();
}
