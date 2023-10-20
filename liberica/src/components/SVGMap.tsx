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
import L, { LeafletMouseEvent } from "leaflet";
import {
  TrainIcon,
  DetectiveIcon,
  MrXIcon,
  ICON_OFFSET,
} from "components/MapIcons";

export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
}

const viewBounds: L.LatLngBounds = new L.LatLngBounds(
  [49.0129685, 8.3782551],
  [48.9906205, 8.4203851]
);

// Game Stuff (Needs to be updated to actually change the game state)
function switchTrain(train: LeafletMouseEvent) {
  console.log("switching train ", train);
}

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

function MapLabel(props: { text: string }) {
  const zoom = useMap().getZoom();

  return <>
    {
      zoom >= 16 && <Tooltip direction="right" offset={ICON_OFFSET} permanent> {props.text} </Tooltip>
    }
  </>;
}

function TrainMarker(props: { train: Train }) {
  const train = props.train;

  return (
    <Marker
      eventHandlers={{ click: switchTrain }}
      icon={TrainIcon}
      position={[train.lat, train.long]}
    >
      <MapLabel text={train.line_name} />
    </Marker>
  );
}

function DetectiveMarker(props: { player: Team }) {
  const player = props.player;
  console.log(player);

  return (
    <Marker icon={DetectiveIcon} position={[player.x, player.y]}>
      <Tooltip> {player.name} </Tooltip>
    </Marker>
  );
}

function MrXMarker(props: { player: Team }) {
  const player = props.player;
  const key = player.id;

  return (
    <Marker icon={MrXIcon} position={[player.x, player.y]}>
      <Tooltip offset={ICON_OFFSET} key={key}>
        {" "}
        Mr X war hier{" "}
      </Tooltip>
    </Marker>
  );
}

export default function SVGMap(props: MapProps) {
  const trains = props.trains ?? [];
  const teams = props.teams ?? [
    {
      id: 1,
      x: 49.012786,
      y: 8.4031014,
      name: "Detective 1",
      color: "#ff0000",
    }
  ];
  const mrX = {
    id: 0,
    x: 49.012796,
    y: 8.4031014,
    name: "Mr X",
    color: "black",
  };

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
              <MrXMarker player={mrX} />
            </LayerGroup>
          </LayersControl.Overlay>
        )}

        {/* Trains */}
        <LayersControl.Overlay checked name="Trains">
          <LayerGroup>
            {props.trains.map((train) => (
              <TrainMarker train={train} key={train.line_id} />        
            ))}
          </LayerGroup>
        </LayersControl.Overlay>

        {/* Detectives */}
        <LayersControl.Overlay checked name="Detectives">
          <LayerGroup>
            {teams.map((player) => (
              <DetectiveMarker player={player} />
            ))}
          </LayerGroup>
        </LayersControl.Overlay>
      </LayersControl>
      <ResetBoundsButton />
    </MapContainer>
  );
}
