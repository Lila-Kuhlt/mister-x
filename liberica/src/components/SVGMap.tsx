import { Train, Team } from "lib/bindings";
import Style from "style/SVGMap.module.css";
import {
  useMap,
  Tooltip,
  Marker,
  MapContainer,
  TileLayer,
} from "react-leaflet";
import { LayersControl, LayerGroup } from "react-leaflet";
import L from "leaflet";
import {
  TrainIcon,
  DetectiveIcon,
  MrXIcon,
  ICON_OFFSET,
  ICON_OFFSET_TOP,
} from "components/MapIcons";
import { WebsocketApi } from "lib/websockts";
import { getContrastingTextColor } from "lib/util";

export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
  ws: WebsocketApi;
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
    <div className="leaflet-ctop leaflet-enter">
      <button className="leaflet-control leaflet-bar" onClick={resetBounds}>
        Reset Map View
      </button>
    </div>
  );
}

function MrXMarker(props: { player: Team; disembark: () => void }) {
  const player = props.player;
  const key = player.id;

  return (
    <Marker
      icon={MrXIcon}
      position={[player.long, player.lat]}
      eventHandlers={{ click: () => props.disembark() }}
    >
      <Tooltip offset={ICON_OFFSET} key={key}>
        {" "}
        Mr X war hier{" "}
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

function DetectiveMarker(props: { player: Team; disembark: () => void }) {
  const player = props.player;
  console.log(player);

  return (
    <Marker
      icon={DetectiveIcon}
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
  const mrX = props.mrX;

  function disembark() {
    props.ws.send({ DisembarkTrain: 0 });
  }

  function embark(train: Train) {
    props.ws.send({ EmbarkTrain: { train_id: train.line_id } });
  }

  return (
    <MapContainer bounds={viewBounds} zoom={13} className={Style.mapContainer}>
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <LayersControl position="topright">
        {/* Mr X */}
        {mrX && (
          <LayersControl.Overlay checked name="Mr X">
            <LayerGroup>
              <MrXMarker player={mrX} disembark={disembark} />
            </LayerGroup>
          </LayersControl.Overlay>
        )}

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
            {teams.map((player) => (
              <DetectiveMarker
                player={player}
                key={player.id}
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
