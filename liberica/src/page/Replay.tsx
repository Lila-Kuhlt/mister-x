import { BASE_URLS, ENDPOINTS } from "lib/api";
import { ReplayMessage, ReplayResponse } from "lib/bindings";
import { WebsocketApi } from "lib/websockets";
import { useGameState, useReplayState, useReplayWebsocketStore } from "lib/state";
import { PropsWithChildren, useEffect } from "react";
import { Map } from "page/Map";
import { Button } from "react-bootstrap";

export function Navbar(props: PropsWithChildren) {
  return (
    <div
      className="position-absolute bottom-0 w-max d-flex bg-white p-2 justify-content-between align-items-center gap-3"
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

export function Replay() {
  const { ws, setWebsocket } = useReplayWebsocketStore();
  const { setGameState } = useGameState();
  const { setTime, setSpeed } = useReplayState();

  useEffect(() => {
    const socket = new WebsocketApi<ReplayResponse, ReplayMessage>(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_REPLAY, setWebsocket)
      .register((msg) => console.log("Received message", msg))
      .register((resp) => {
        if (resp !== "End") {
          setGameState(JSON.parse(resp.Frame.game_state));
          setTime(resp.Frame.time);
        }
      })
    return () => socket.disconnect();
  }, [setGameState, setWebsocket]);

  useEffect(() => {
    if (!ws) return;

    ws.send({ Speed: 10.0 });
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
      </Navbar>
    </>
  ) : (
    <WarningPage>
      <h3>Server is reloading</h3>
      <p>Reload the site in a few seconds</p>
    </WarningPage>
  );
}
