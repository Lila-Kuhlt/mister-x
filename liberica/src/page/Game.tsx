import { Map } from "components/map/Map";
import { createWebsocketConnection } from "lib/api";
import { GameState } from "lib/bindings";
import { WebsocketApi } from "lib/websockets";
import { Popup } from "react-leaflet";
import { createContext, useEffect, useState } from "react";
import { Marker } from "components/map/Marker";

export const WebsocketContext = createContext<WebsocketApi | undefined>(
  undefined
);

export function Game() {
  const [ws, setWS] = useState<WebsocketApi | undefined>();
  const [gs, setGameState] = useState<GameState>({ teams: [], trains: [] });

  useEffect(() => {
    const socket = createWebsocketConnection();

    const onClose = (e: Event) => {
      setWS(undefined);
      console.error(`Websocket connection closed uncleanly: `, e);
      setTimeout(() => socket.reconnect(), 1000);
    };

    socket
      .registerEvent("Connect", () => setWS(socket))
      .registerEvent("Error", (e) => onClose(e))
      .registerEvent("Disconnect", () => setWS(undefined));

    socket.register("GameState", (gs) => setGameState(gs));

    return () => socket.disconnect();
  }, []);

  // Loading page
  const LOADER = () => (
    <div className="flex flex-col items-center justify-center gap-5 w-max h-max">
      <div className="flex flex-col items-center">
        <span className="italic text-slate-400">
          Connection to game server lost
        </span>
        <span className="italic text-slate-400">
          Attempting to reconnect...
        </span>
      </div>
    </div>
  );

  const MAP = () => (
    <WebsocketContext.Provider value={ws}>
      <Map
        tileProps={{ updateInterval: 500 }}
        containerProps={{ preferCanvas: true }}
      >
        {gs.trains.map((train) => (
          <Marker key={train.line_id} position={{ ...train }}>
            <Popup>{train.line_name}</Popup>
          </Marker>
        ))}
      </Map>
    </WebsocketContext.Provider>
  );

  return ws ? MAP() : LOADER();
}
