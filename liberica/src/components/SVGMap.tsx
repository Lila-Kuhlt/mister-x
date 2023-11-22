import { Button } from "react-bootstrap";
import { Train, Team, Stop } from "lib/bindings";
import Style from "style/SVGMap.module.css";
import {
  useMap,
  Tooltip,
  Marker,
  MapContainer,
  TileLayer,
} from "react-leaflet";
import { LayersControl, LayerGroup, Circle } from "react-leaflet";
import L from "leaflet";
import {
  TrainIcon,
  DetectiveIcon,
  MrXIcon,
  ICON_OFFSET,
  ICON_OFFSET_TOP,
} from "components/MapIcons";
import { getContrastingTextColor } from "lib/util";
import { useGameState, useGameWebsocketStore, useTeamStore } from "lib/state";

export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
  stops: Stop[];
}

const viewBounds: L.LatLngBounds = new L.LatLngBounds(
  [49.0129685, 8.3782551],
  [48.9906205, 8.4203851]
);

// React Stuff
function ResetBoundsButton() {
  const resetBounds = () => {
    map.fitBounds(viewBounds);
  };
  const map = useMap();

  return (
    <div className="leaflet-top leaflet-center">
      <Button className="leaflet-control leaflet-bar" disabled={false} variant="primary" onClick={resetBounds}>
        Reset Map View
      </Button>
    </div>
  );
}

function MrXMarker(props: { player: Team; disembark: () => void }) {
  const player = props.player;
  const key = player.id;

  return (
    <Marker
      icon={MrXIcon}
      position={[player.lat, player.long]}
      eventHandlers={{ click: () => props.disembark() }}
    >
      <Tooltip offset={ICON_OFFSET} key={key}>
        {" "}
        Mr. X war hier{" "}
      </Tooltip>
    </Marker>
  );
}

function TrainMarker(props: {
  train: Train;
  embarked: boolean;
  embark: (train: Train) => void;
  disembark: () => void;
}) {
  const train = props.train;
  const zoom = useMap().getZoom();

  return (
    <Marker
      eventHandlers={{
        click: () =>
          !props.embarked ? props.embark(train) : props.disembark(),
      }}
      icon={TrainIcon}
      position={[train.lat, train.long]}
    >
      {zoom >= 16 && (
        <Tooltip direction="right" offset={ICON_OFFSET} permanent>
          {" "}
          {train.line_name.split(" ")[1]} to {train.direction}{" "}
        </Tooltip>
      )}
    </Marker>
  );
}

function TeamMarker(props: { player: Team; disembark: () => void }) {
  const player = props.player;

  return (
    <Marker
      icon={player.mr_x ? MrXIcon : DetectiveIcon}
      position={[player.lat, player.long]}
      eventHandlers={{ click: () => props.disembark() }}
    >
      <Tooltip
        className={Style.tooltip}
        direction="top"
        opacity={1}
        offset={ICON_OFFSET_TOP}
        permanent
      >
        <a
          style={{
            background: player.color,
            color: getContrastingTextColor(player.color),
          }}
          className={Style.detectiveLabel}
        >
          {player.name}
        </a>
      </Tooltip>
    </Marker>
  );
}

export default function SVGMap(props: MapProps) {
  const trains = props.trains;
  const teams = props.teams;
  const stops = props.stops;
  const mrX = props.mrX;
  const { setEmbarkedTrain } = useGameState();
  const { ws } = useGameWebsocketStore();
  const TS = useTeamStore();

  function disembark() {
    if (TS.team) {
      setEmbarkedTrain(undefined);
      ws?.send({ DisembarkTrain: 0 });
    }
  }

  function embark(train: Train) {
    if (TS.team) {
      setEmbarkedTrain(train);
      ws?.send({ EmbarkTrain: { train_id: train.line_id } });
    }
  }

  return (
    <MapContainer bounds={viewBounds} zoom={13} className={Style.mapContainer}>
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <LayersControl position="topright">
        {/* Stops */}
        <LayersControl.Overlay checked name="Stops">
          <LayerGroup>
            {stops.map((stop) => (
              <Circle
                eventHandlers={{
                  click: () => TS.team && ws?.send({ SetTeamPosition: { lat: stop.lat, long: stop.lon, team_id: TS.team.id } }),
                }}
                center={[stop.lat, stop.lon]}
                pathOptions={{
                  color: "none",
                  fillColor: "blue",
                  opacity: 10.0,
                }}
                radius={50}
                key={stop.id}
              />
            ))}
          </LayerGroup>
        </LayersControl.Overlay>

        {/* Trains */}
        <LayersControl.Overlay checked name="Trains">
          <LayerGroup>
            {trains.map((train) => (
              <TrainMarker
                disembark={disembark}
                embark={embark}
                train={train}
                key={train.line_id}
                embarked={
                  teams.find((team) => team.on_train === train.line_id) !==
                  undefined
                }
              />
            ))}
          </LayerGroup>
        </LayersControl.Overlay>

        {/* Detectives */}
        <LayersControl.Overlay checked name="Detectives">
          <LayerGroup>
            {teams
              .filter((team) => team.lat !== 0.0 || team.long !== 0.0) // sensible coordinates
              .filter((team) => !team.mr_x || TS.team?.mr_x) // team is not Mr. X or the client is Mr. X
              .map((team) => (
                <TeamMarker
                  player={team}
                  key={team.id}
                  disembark={disembark}
                />
              ))}
          </LayerGroup>
        </LayersControl.Overlay>
      </LayersControl>
      <ResetBoundsButton />
    </MapContainer>
  );
}
