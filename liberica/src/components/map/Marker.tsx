import { PropsWithChildren } from "react";
import { Marker as LMarker, Tooltip, useMap } from "react-leaflet";
import { Icon } from "leaflet";
import {
  MrXIcon,
  ICON_OFFSET,
  TrainIcon,
  DetectiveIcon,
  ICON_OFFSET_TOP,
} from "components/MapIcons";
import { TeamState, Train } from "lib/bindings";
import { getContrastingTextColor } from "lib/util";
import Style from "style/Map.module.css";

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
      eventHandlers={{ click: () => props.onClick?.() }}
    >
      {props.children}
    </LMarker>
  );
}

export function MrXMarker(props: { player: TeamState }) {
  const player = props.player;

  return (
    <Marker icon={MrXIcon} position={{ ...player }}>
      <Tooltip offset={ICON_OFFSET} key={player.team.id}>
        Mr. X war hier
      </Tooltip>
    </Marker>
  );
}

export function TrainMarker(props: {
  train: Train;
  onClick?: (train: Train) => void;
}) {
  const train = props.train;
  const zoom = useMap().getZoom();

  return (
    <Marker
      icon={TrainIcon}
      position={{ ...train }}
      onClick={() => props.onClick?.(train)}
    >
      {zoom > 15 && (
        <Tooltip direction="right" offset={ICON_OFFSET} permanent>
          {train.line_name.split(" ")[1]} to {train.direction}
        </Tooltip>
      )}
    </Marker>
  );
}

export function TeamMarker(props: { player: TeamState }) {
  const player = props.player;

  return (
    <Marker
      icon={player.team.kind == "MrX" ? MrXIcon : DetectiveIcon}
      position={{ ...player }}
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
            background: player.team.color,
            color: getContrastingTextColor(player.team.color),
          }}
          className={Style.detectiveLabel}
        >
          {player.team.name}
        </a>
      </Tooltip>
    </Marker>
  );
}
