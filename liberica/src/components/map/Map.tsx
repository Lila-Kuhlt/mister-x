import {
  TileLayerProps,
  MapContainerProps,
  MapContainer,
  TileLayer,
} from "react-leaflet";

const CENTER: [number, number] = [49.0046, 8.403];

export type MapProps = React.PropsWithChildren<{
  tileProps?: Partial<TileLayerProps>;
  containerProps?: Partial<MapContainerProps>;
}>;

export function Map(props: MapProps) {
  return (
    <MapContainer
      center={CENTER}
      zoom={15}
      className="h-max w-max"
      zoomControl={false}
      {...props.containerProps}
    >
      <TileLayer
        url="https://cartodb-basemaps-{s}.global.ssl.fastly.net/light_all/{z}/{x}/{y}.png"
        updateInterval={200}
        {...props.tileProps}
      />
      {props.children}
    </MapContainer>
  );
}
