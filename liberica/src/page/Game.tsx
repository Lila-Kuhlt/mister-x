import { GameStateContext, Map } from "components/map/Map";
import { createWebsocketConnection } from "lib/api";
import { GameState, Team, Train } from "lib/bindings";
import { WebsocketApi } from "lib/websockets";
import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Button } from "components/InputElements";

export function Game() {
  const [ws, setWS] = useState<WebsocketApi>();
  const [gs, setGameState] = useState<GameState>({ teams: [], trains: [] });
  const [embarkedTrain, setEmbarkedTrain] = useState<Train>();
  const team: Team = useLocation().state; // this is how Home passes the team
  const navigate = useNavigate();

  function disembark() {
    if (team) {
      setEmbarkedTrain(undefined);
      ws?.send("DisembarkTrain");
    }
  }

  function embark(train: Train) {
    if (team) {
      setEmbarkedTrain(train);
      ws?.send({ EmbarkTrain: { train_id: train.line_id } });
    }
  }

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

  useEffect(() => {
    if (ws && team) {
      ws.send({ JoinTeam: { team_id: team.id } })
    }
  }, [ws, team]);

  useEffect(() => {
    if (ws && window.isSecureContext) {
      navigator.geolocation.watchPosition((pos) => {
        ws.send({ Position: { lat: pos.coords.latitude, long: pos.coords.longitude } });
      });
    }
  }, [ws]);

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
    <GameStateContext.Provider value={gs}>
      <Map
        tileProps={{ updateInterval: 500 }}
        containerProps={{ preferCanvas: true }}
        onStopClick={(stop) => ws?.send({ SetTeamPosition: { lat: stop.lat, long: stop.lon } })}
        onTrainClick={(train) => {
          const embarked = gs.teams.find((team) => team.on_train === train.line_id) !== undefined;
          if (embarked) {
            disembark();
          } else {
            embark(train);
          }
        }}
      />
    </GameStateContext.Provider>
  );

  return ws ? MAP() : LOADER();
}
