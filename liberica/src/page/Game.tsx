import { BASE_URLS, ENDPOINTS } from "lib/api";
import { ClientMessage, GameState } from "lib/bindings";
import { WebsocketApi } from "lib/websockets";
import { useGameState, useTeamStore, useGameWebsocketStore } from "lib/state";
import { PropsWithChildren, useEffect } from "react";
import { Map } from "page/Map";
import { Button } from "react-bootstrap";
import { Navbar } from "components/Navbar";

export function Game() {
  const { ws, setWebsocket } = useGameWebsocketStore();
  const { setGameState, embarkedTrain, setEmbarkedTrain } = useGameState();
  const TS = useTeamStore();

  useEffect(() => {
    const socket = new WebsocketApi<GameState, ClientMessage>(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS, setWebsocket)
      .register((msg) => console.log("Received message", msg))
      .register(setGameState)
    return () => socket.disconnect();
  }, [setGameState, setWebsocket]);

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

  return ws ? (
    <>
      <Map />
      <Navbar>
        <Button
          onClick={() => {
            window.location.href = "/";
          }}
        >
          <i className="bi bi-house-fill"></i>
        </Button>
        {embarkedTrain && (
          <span>
            {embarkedTrain?.line_name} {embarkedTrain?.direction}
          </span>
        )}
        <Button
          disabled={!embarkedTrain}
          onClick={() => {
            ws.send({ DisembarkTrain: 0 });
            setEmbarkedTrain(undefined);
          }}
        >
          Disembark
        </Button>
      </Navbar>
    </>
  ) : (
    <div className="d-flex flex-center w-max h-max flex-column">
      <h3>Server is reloading</h3>
      <p>Reload the site in a few seconds</p>
    </div>
  );
}
