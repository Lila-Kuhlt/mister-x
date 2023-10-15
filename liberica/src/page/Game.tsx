import { ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useEffect, useState } from "react";
import { Map } from "page/Map";
import { GameState } from "lib/bindings";


export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);
  const [gameState, setGameState] = useState<GameState | undefined>(undefined);

  async function updateGame(message: GameState) {
    console.log("update game: ", gameState);
    setGameState(message);
  }

  useEffect(() => {
    new WebsocketApi(ENDPOINTS.GET_WS, setWs).register(console.log).register(updateGame);
  }, []);

  useEffect(() => ws?.send({ Message: "lol" }), [ws]);

  useEffect(() => {
    navigator.geolocation.watchPosition(
      (pos) =>
        pos.coords.altitude &&
        ws?.send({
          Position: { x: pos.coords.altitude, y: pos.coords.longitude },
        })
    );
  }, [ws]);
  console.log("render loop: ", gameState);

  return <div>{gameState && <Map trains={gameState.trains} teams={Object.values(gameState.teams)} />}</div>;
}
