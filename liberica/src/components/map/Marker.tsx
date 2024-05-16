import { PropsWithChildren } from 'react';
import { Marker as LMarker } from 'react-leaflet';
import { Icon } from 'leaflet';

export function Marker(
    props: PropsWithChildren & {
        icon: Icon;
        position: { lat: number; long: number };
        onClick?: () => void;
    }
) {
    return (
        <LMarker
            icon={props.icon}
            position={{ lat: props.position.lat, lng: props.position.long }}
            eventHandlers={{ click: () => props.onClick?.() }}>
            {props.children}
        </LMarker>
    );
}
