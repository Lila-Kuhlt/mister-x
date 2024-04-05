import {
  useMap,
  TileLayerProps,
  MapContainerProps,
  MapContainer,
  TileLayer,
  LayersControl,
  LayerGroup,
  Circle,
} from "react-leaflet";
import { createContext, useContext, useEffect, useState } from "react";
import { Button } from "components/InputElements";
import { GameState, Stop, Train } from "lib/bindings";
import { getStops } from "lib/api";
import { TrainMarker, TeamMarker } from "./Marker";

export const GameStateContext = createContext<GameState>({
  teams: [],
  trains: [],
});

const CENTER: [number, number] = [49.0046, 8.403];
const DEFAULT_ZOOM: number = 15;

export type MapProps = React.PropsWithChildren<{
  tileProps?: Partial<TileLayerProps>;
  containerProps?: Partial<MapContainerProps>;
  onStopClick?: (stop: Stop) => void;
  onTrainClick?: (train: Train) => void;
}>;

function ResetMapViewButton() {
  const map = useMap();

  return (
    <div className="leaflet-top leaflet-center">
      <div className="leaflet-control leaflet-bar">
        <Button onClick={() => map.setView(CENTER, DEFAULT_ZOOM)}>
          Reset Map View
        </Button>
      </div>
    </div>
  );
}

export function Map(props: MapProps) {
  const gs = useContext(GameStateContext);
  const [stops, setStops] = useState<Stop[]>([]);
  useEffect(() => {
    getStops().then(setStops);
  }, []);

  return (
    <MapContainer
      center={CENTER}
      zoom={DEFAULT_ZOOM}
      className="h-max w-max"
      zoomControl={false}
      {...props.containerProps}
    >
      <TileLayer
        url="https://cartodb-basemaps-{s}.global.ssl.fastly.net/light_all/{z}/{x}/{y}.png"
        updateInterval={200}
        {...props.tileProps}
      />
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
      <ResetMapViewButton />
      {props.children}
    </MapContainer>
  );
}
