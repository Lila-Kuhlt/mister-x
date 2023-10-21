import { BASE_URLS, ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useEffect, useState } from "react";
import { Map } from "page/Map";
import { GameState } from "lib/bindings";

export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);
  const [gameState, setGameState] = useState<GameState | undefined>(undefined);

  useEffect(() => {
    new WebsocketApi(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS, setWs)
      .register((msg) => console.log("Received message", msg))
      .register(setGameState);
  }, []);

  useEffect(() => ws?.send({ Message: "Hello from Client" }), [ws]);

  useEffect(() => {
    if (!window.isSecureContext) return;
    navigator.geolocation.watchPosition(
      (pos) =>
        pos.coords.altitude &&
        ws?.send({
          Position: { lat: pos.coords.latitude, long: pos.coords.longitude },
        })
    );
  }, [ws]);

  return (
    <Map gameState={gameState} />
  );
}
