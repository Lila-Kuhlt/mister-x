import { BASE_URLS, ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useTeamStore } from "lib/state";
import { useEffect, useState } from "react";
import { Map } from "page/Map";
import { GameState } from "lib/bindings";

export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);
  const [gameState, setGameState] = useState<GameState | undefined>(undefined);
  const TS = useTeamStore();

  useEffect(() => {
    new WebsocketApi(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS, setWs)
      .register((msg) => console.log("Received message", msg))
      .register(setGameState);
  }, []);

  useEffect(() => {
    if (!ws) return;
    if (!TS.team) return;

    ws.send({ Message: "Hello from Client" });
    ws.send({ JoinTeam: { team_id: TS.team.id } });
  }, [ws, TS.team]);

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

  return ws && <Map gameState={gameState} ws={ws} />;
}
