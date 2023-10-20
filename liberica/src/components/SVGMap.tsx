import { Train, Team } from "lib/bindings";
import Style from "style/SVGMap.module.css";
import {
  useMap,
  Tooltip,
  Marker,
  MapContainer,
  TileLayer,
} from "react-leaflet";
import { LayersControl } from "react-leaflet";
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
  const tooltipText = `Linie S100 to Durlacher Tor/KIT-Campus Süd`;

  return (
    <Marker
      eventHandlers={{ click: switchTrain }}
      icon={TrainIcon}
      position={[train.lat, train.long]}
    >
      <MapLabel text={tooltipText} />
    </Marker>
  );
}

function DetectiveMarker(props: { player: Team }) {
  const player = props.player;

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
  const teams = props.teams ?? [];
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
        {props.mrX && (
          <LayersControl.Overlay checked name="Mr X">
            <Marker icon={MrXIcon} position={[props.mrX.x, props.mrX.y]}>
              <Tooltip> Mr X war hier </Tooltip>
            </Marker>
          </LayersControl.Overlay>
        )}

        {/* Trains */}
        <LayersControl.Overlay checked name="Trains">
          {props.trains.map((train) => (
            // TODO: Change key to trian.id, when backend responds with correct data
            <TrainMarker train={train} key={train.direction} />
          ))}
        </LayersControl.Overlay>

        {/* Detectives */}
        <LayersControl.Overlay checked name="Detectives">
          {props.teams.map((player) => (
            <DetectiveMarker player={player} />
          ))}
        </LayersControl.Overlay>
      </LayersControl>
      <ResetBoundsButton />
    </MapContainer>
  );
}
