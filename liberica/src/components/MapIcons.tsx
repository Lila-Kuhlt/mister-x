import trainSVG from "assets/tram2.svg";
import detectiveSVG from "assets/tie.svg";
import mrXSVG from "assets/secret.svg";
import TramSVG from "assets/tram1.svg";

import L, { IconOptions } from "leaflet";

export const ICON_OFFSET: [number, number] = [15, 0];
export const ICON_OFFSET_TOP: [number, number] = [0, -15];

const DEFAULT_SETTINGS: Partial<IconOptions> = {
  iconSize: [30, 30], // size of the icon
  shadowSize: [0, 0], // size of the shadow
  iconAnchor: [15, 15], // point of the icon which will correspond to marker's location
  shadowAnchor: [0, 0], // the same for the shadow
  popupAnchor: [0, 0], // point from which the popup should open relative to the iconAnchor
};

export const TrainIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: trainSVG,
});

export const DetectiveIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: detectiveSVG,
});

export const MrXIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: mrXSVG,
});

export const PersonIcon = new L.Icon({
  iconUrl: TramSVG,
  iconRetinaUrl: TramSVG,
  iconSize: new L.Point(60, 75),
  className: "leaflet-div-icon",
});
