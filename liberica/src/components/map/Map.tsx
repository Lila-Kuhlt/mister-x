import {
    useMap,
    TileLayerProps,
    MapContainerProps,
    MapContainer,
    TileLayer,
    LayersControl,
    LayerGroup,
    Circle,
    Tooltip,
} from "react-leaflet";
import { createContext, useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { MrXIcon, TrainIcon, DetectiveIcon } from "components/MapIcons";
import { Button } from "components/InputElements";
import { Marker } from "./Marker";
import { GameState, Stop, TeamState, Train } from "lib/bindings";
import { getStops } from "lib/api";
import { getContrastingTextColor } from "lib/util";
import Style from "style/Map.module.css";
import React from "react";

export const GameStateContext = createContext<GameState>({
    teams: [],
    trains: [],
});

const CENTER: [number, number] = [49.0046, 8.403];
const DEFAULT_ZOOM = 15;

export type MapProps = React.PropsWithChildren<{
    tileProps?: Partial<TileLayerProps>;
    containerProps?: Partial<MapContainerProps>;
}>;

function ResetMapViewButton() {
    const map = useMap();
    const { t } = useTranslation();

    return (
        <div className="leaflet-top leaflet-center">
            <div className="leaflet-control leaflet-bar">
                <Button onClick={() => map.setView(CENTER, DEFAULT_ZOOM)}>
                    {t("ResetMapView")}
                </Button>
            </div>
        </div>
    );
}

function TrainMarker(props: {
    train: Train;
    onClick?: (train: Train) => void;
}) {
    const train = props.train;
    const zoom = useMap().getZoom();
    const { t } = useTranslation();

    return (
        <Marker
            icon={TrainIcon}
            position={{ ...train }}
            onClick={() => props.onClick?.(train)}
        >
            {zoom > DEFAULT_ZOOM && (
                <Tooltip direction="right" permanent>
                    {t("to", {
                        line: train.line_name.split(" ")[1],
                        direction: train.direction,
                    })}
                </Tooltip>
            )}
        </Marker>
    );
}

function TeamMarker(props: { player: TeamState }) {
    const player = props.player;

    return (
        <Marker
            icon={player.team.kind == "MrX" ? MrXIcon : DetectiveIcon}
            position={{ ...player }}
        >
            <Tooltip
                className={Style.tooltip}
                direction="top"
                opacity={1}
                permanent
            >
                <a
                    style={{
                        background: player.team.color,
                        color: getContrastingTextColor(player.team.color),
                    }}
                    className={Style.detectiveLabel}
                >
                    {player.team.name}
                </a>
            </Tooltip>
        </Marker>
    );
}

export function Map(
    props: MapProps & {
        onStopClick?: (stop: Stop) => void;
        onTrainClick?: (train: Train) => void;
    },
) {
    const gs = useContext(GameStateContext);
    const [stops, setStops] = useState<Stop[]>([]);
    useEffect(() => {
        getStops()
            .then(setStops)
            .catch(() => console.warn("failed to get stops"));
    }, []);

    return (
        <MapContainer
            center={CENTER}
            zoom={DEFAULT_ZOOM}
            className="h-max w-max"
            {...props.containerProps}
        >
            <TileLayer
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                updateInterval={200}
                {...props.tileProps}
            />
            <LayersControl position="topright">
                {/* Stops */}
                <LayersControl.Overlay checked name="Stops">
                    <LayerGroup>
                        {stops.map((stop) => (
                            <Circle
                                key={stop.id}
                                eventHandlers={{
                                    click: () => props.onStopClick?.(stop),
                                }}
                                center={[stop.lat, stop.lon]}
                                pathOptions={{
                                    color: "none",
                                    fillColor: "blue",
                                    opacity: 10.0,
                                }}
                                radius={50}
                            />
                        ))}
                    </LayerGroup>
                </LayersControl.Overlay>

                {/* Trains */}
                <LayersControl.Overlay checked name="Trains">
                    <LayerGroup>
                        {gs.trains.map((train) => (
                            <TrainMarker
                                train={train}
                                key={train.line_id}
                                onClick={() => props.onTrainClick?.(train)}
                            />
                        ))}
                    </LayerGroup>
                </LayersControl.Overlay>

                {/* Detectives */}
                <LayersControl.Overlay checked name="Detectives">
                    <LayerGroup>
                        {gs.teams
                            .filter((ts) => ts.lat !== 0.0 || ts.long !== 0.0) // sensible coordinates
                            .map((ts) => (
                                <TeamMarker player={ts} key={ts.team.id} />
                            ))}
                    </LayerGroup>
                </LayersControl.Overlay>
            </LayersControl>
            <ResetMapViewButton />
            {props.children}
        </MapContainer>
    );
}
