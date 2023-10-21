import { BASE_URLS, ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useGameState, useTeamStore } from "lib/state";
import { PropsWithChildren, useEffect, useState } from "react";
import { Map } from "page/Map";
import { Button } from "react-bootstrap";

export function Navbar(props: PropsWithChildren) {
  return (
    <div
      className="position-absolute bottom-0 w-max d-flex bg-white p-2 justify-content-between align-items-center"
      style={{ zIndex: 10000 }}
    >
      {props.children}
    </div>
  );
}

export function WarningPage(props: PropsWithChildren) {
  return (
    <div className="d-flex flex-center w-max h-max flex-column">
      {props.children}
    </div>
  );
}

export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);
  const { setGameState, embarkedTrain } = useGameState();
  const TS = useTeamStore();

  useEffect(() => {
    new WebsocketApi(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS, setWs)
      .register((msg) => console.log("Received message", msg))
      .register(setGameState)
      .setDisconnectHandler(() => setTimeout(() => location.reload(), 5000));
  }, [setGameState]);

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
      <Map ws={ws} />
      <Navbar>
        <Button
          onClick={() => {
            TS.setTeam(undefined);
            window.location.href = "/";
          }}
        >
          <i className="bi bi-house-fill"></i>
        </Button>
        {embarkedTrain && (
          <span>
            Current: {embarkedTrain?.line_name} {embarkedTrain?.direction}
          </span>
        )}
        {/* <Button disabled={!TS.team}>Disembark</Button> */}
      </Navbar>
    </>
  ) : (
    <WarningPage>
      <h3>Server is reloading</h3>
      <p>Reload the site in a few seconds</p>
    </WarningPage>
  );
}
