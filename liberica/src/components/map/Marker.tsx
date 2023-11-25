import { PropsWithChildren } from "react";
import { Marker as LMarker } from "react-leaflet";

export function Marker(
  props: PropsWithChildren & { position: { lat: number; long: number } }
) {
  return (
    <LMarker
      position={{ lat: props.position.lat, lng: props.position.long }}
    ></LMarker>
  );
}
