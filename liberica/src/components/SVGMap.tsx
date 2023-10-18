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
import L from "leaflet";
import { TrainIcon, DetectiveIcon, MrXIcon } from "components/MapIcons";
export interface MapProps { trains: Train[], teams: Team[], mrX?: Team }

const viewBounds: L.LatLngBounds = new L.LatLngBounds(
  [49.0129685, 8.3782551],
  [48.9906205, 8.4203851]
);

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


export default function SVGMap(props: MapProps) {
  const trains = props.trains | [];
  const teams = props.teams | [];
  // Mr X is hardcoded for now
  const mrX =  { id: 0, x: 49.012796, y: 8.4031014, name: "Mr X", color: "black" };
  
  return (
    <MapContainer bounds={viewBounds} zoom={13} className={Style.mapContainer}>
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <Marker icon={MrXIcon} position={[props.mrX.x, props.mrX.y]}>
        <Tooltip> Mr X war hier </Tooltip>
      </Marker>
      <LayersControl position="topright">
        {/* Mr X */}
        <LayersControl.Overlay checked name="Mr X">
          <Marker icon={MrXIcon} position={[props.mrX.x, props.mrX.y]}>
            <Tooltip> Mr X war hier </Tooltip>
          </Marker>
        </LayersControl.Overlay>

        {/* Trains */}
        <LayersControl.Overlay checked name="Trains">
          {props.trains.map((train) => (
            <Marker
              key={train.lat + train.line_id + train.long}
              icon={TrainIcon}
              position={[train.lat, train.long]}
            >
              <Tooltip>
                {" "}
                Linie {train.line_id} to {train.direction}{" "}
              </Tooltip>
            </Marker>
          ))}
        </LayersControl.Overlay>

        {/* Detectives */}
        <LayersControl.Overlay checked name="Detectives">
          {props.teams.map((player) => (
            <Marker
              key={player.id}
              icon={DetectiveIcon}
              position={[player.x, player.y]}
            >
              <Tooltip> {player.name} </Tooltip>
            </Marker>
          ))}
        </LayersControl.Overlay>
      </LayersControl>
      <ResetBoundsButton />
    </MapContainer>
  );
}
