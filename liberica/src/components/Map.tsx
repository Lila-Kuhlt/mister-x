import { MapContainer, TileLayer } from "react-leaflet";

const CENTER: [number, number] = [49.0046, 8.403];

export function Map() {
  return (
    <MapContainer center={CENTER} zoom={15} className="h-max w-max">
      <TileLayer
        url="https://cartodb-basemaps-{s}.global.ssl.fastly.net/light_all/{z}/{x}/{y}.png"
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
      />
    </MapContainer>
  );
}
