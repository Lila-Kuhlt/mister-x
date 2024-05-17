import { GameStateContext, Map } from "components/map/Map";
import { createWebSocketConnection } from "lib/api";
import { GameState, Team, Train } from "lib/bindings";
import { WebSocketApi } from "lib/websockets";
import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Navbar } from "components/Navbar";
import { Button } from "components/InputElements";
import { FaHome } from "react-icons/fa";
import { useTranslation } from "react-i18next";

export function Game() {
    const [ws, setWS] = useState<WebSocketApi>();
    const [gs, setGameState] = useState<GameState>({ teams: [], trains: [] });
    const [embarkedTrain, setEmbarkedTrain] = useState<Train>();
    const team: Team = useLocation().state; // this is how Home passes the team
    const navigate = useNavigate();
    const { t } = useTranslation();

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
        const socket = createWebSocketConnection();

        socket
            .registerEvent("Connect", () => setWS(socket))
            .registerEvent("Error", (e) => {
                setWS(undefined);
                console.error("WebSocket connection closed uncleanly:", e);
                setTimeout(() => socket.reconnect(), 1000);
            });

        socket.register("GameState", (gs) => setGameState(gs));

        return () => {
            setWS(undefined);
            socket.disconnect();
        };
    }, []);

    useEffect(() => {
        if (team) {
            ws?.send({ JoinTeam: { team_id: team.id } });
        }
    }, [ws, team]);

    useEffect(() => {
        if (window.isSecureContext) {
            navigator.geolocation.watchPosition((pos) => {
                ws?.send({
                    Position: {
                        lat: pos.coords.latitude,
                        long: pos.coords.longitude,
                    },
                });
            });
        }
    }, [ws]);

    const Game = (
        <GameStateContext.Provider value={gs}>
            <div className="flex h-max w-max flex-col">
                <Map
                    tileProps={{ updateInterval: 500 }}
                    containerProps={{ preferCanvas: true }}
                    onStopClick={(stop) => {
                        if (team) {
                            disembark();
                            ws?.send({
                                SetTeamPosition: {
                                    lat: stop.lat,
                                    long: stop.lon,
                                },
                            });
                        }
                    }}
                    onTrainClick={embark}
                />

                <Navbar>
                    <Button onClick={() => navigate("/")}>
                        <FaHome />
                    </Button>

                    {embarkedTrain && (
                        <span>
                            {embarkedTrain.line_name} {embarkedTrain.direction}
                        </span>
                    )}

                    <Button disabled={!embarkedTrain} onClick={disembark}>
                        {t("Disembark")}
                    </Button>
                </Navbar>
            </div>
        </GameStateContext.Provider>
    );

    const LandingPage = (
        <div className="flex h-max w-max flex-col items-center justify-center gap-5">
            <div className="flex flex-col items-center">
                <span className="italic text-slate-400">
                    {t("ConnectionLost")}
                </span>
                <span className="italic text-slate-400">{t("Reconnect")}</span>
            </div>
        </div>
    );

    return ws ? Game : LandingPage;
}
