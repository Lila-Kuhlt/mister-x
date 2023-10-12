import { Train, Player} from 'lib/bindings'
import "./SVGMap.css"
import { Tooltip, Marker, MapContainer, TileLayer } from 'react-leaflet'
import { LayersControl } from 'react-leaflet'
import L from 'leaflet';
import { TrainIcon, DetectiveIcon, MrXIcon } from './map_icons'

var viewBounds: L.LatLngBounds = new L.LatLngBounds([49.0129685,8.3782551], [48.9906205,8.4203851]);

export default function SVGMap(props: { trains: Train[], players: Player[], mrX: Player}) {
  return <div> 
    <MapContainer bounds={viewBounds} zoom={13} >
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
       />
     <Marker 
       icon={MrXIcon}
       position={[props.mrX.x, props.mrX.y]}>
       <Tooltip> Mr X war hier </Tooltip>
     </Marker>
     <LayersControl position="topright">
        {/* Mr X */}
        <LayersControl.Overlay checked name="Mr X">
          <Marker 
            icon={MrXIcon}
            position={[props.mrX.x, props.mrX.y]}>
            <Tooltip> Mr X war hier </Tooltip>
          </Marker>
        </LayersControl.Overlay>

        {/* Trains */}
        <LayersControl.Overlay checked name="Trains">
         {props.trains.map((train) => 
           <Marker 
             key={train.id} 
             icon={TrainIcon}
             position={[train.lat, train.long]}>
             <Tooltip> Linie {train.line_id} to {train.direction} </Tooltip>
           </Marker>
         )}
        </LayersControl.Overlay>

        {/* Detectives */}
        <LayersControl.Overlay checked name="Detectives">
        {props.players.map((player) =>
          <Marker
            key={player.id}
            icon={DetectiveIcon}
            position={[player.x, player.y]}>
            <Tooltip> {player.name} </Tooltip>
          </Marker>
        )}
        </LayersControl.Overlay>
       </LayersControl> 
     </MapContainer>
   </div>
}
