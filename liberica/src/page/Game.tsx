import { GameStateContext, Map } from "components/map/Map";
import { createWebSocketConnection } from "lib/api";
import { GameState, Team, Train } from "lib/bindings";
import { WebSocketApi } from "lib/websockets";
import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { Navbar } from "components/Navbar";
import { Button } from "components/lila/button";
import { useTranslation } from "react-i18next";

export function Game() {
    const [ws, setWS] = useState<WebSocketApi>();
    const [gs, setGameState] = useState<GameState>({ teams: [], trains: [] });
    const [embarkedTrain, setEmbarkedTrain] = useState<Train>();
    const team = useLocation().state as Team | undefined; // this is how Home passes the team
    const { t } = useTranslation();

    function disembark() {
        if (!team) return;

        setEmbarkedTrain(undefined);
        ws?.send("DisembarkTrain");
    }

    function embark(train: Train) {
        if (!team) return;

        setEmbarkedTrain(train);
        ws?.send({ EmbarkTrain: { train_id: train.line_id } });
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
        if (!team) return;
        ws?.send({ JoinTeam: { team_id: team.id } });
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
        <div className="flex h-dvh w-dvw flex-col">
            <GameStateContext.Provider value={gs}>
                <Map
                    tileProps={{ updateInterval: 500 }}
                    containerProps={{ preferCanvas: true, zoomControl: false }}
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
                    <div className="flex h-full w-full flex-col truncate rounded-xl rounded-xl bg-muted/10 px-4 py-2.5 text-sm text-on-muted">
                        {embarkedTrain ? (
                            <span className="truncate">
                                {embarkedTrain.line_name}{" "}
                                {embarkedTrain.direction}
                            </span>
                        ) : (
                            <span className="truncate italic text-on-muted/50">
                                {t("EmbarkPlaceholder")}
                            </span>
                        )}
                    </div>

                    <Button
                        disabled={!embarkedTrain}
                        onClick={disembark}
                        variant={"primary"}
                        size="lg"
                    >
                        {t("Disembark")}
                    </Button>
                </Navbar>
            </GameStateContext.Provider>
        </div>
    );

    return ws ? Game : <LandingPage />;
}

function LandingPage() {
    const { t } = useTranslation();

    return (
        <div className="flex h-dvh w-dvw flex-col items-center justify-center gap-5">
            <div className="flex flex-col items-center">
                <span className="italic text-on-base">
                    {t("ConnectionLost")}
                </span>
                <span className="italic text-on-base">{t("Reconnect")}</span>
            </div>
        </div>
    );
}
