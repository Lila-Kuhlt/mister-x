import trainSVG from "assets/tram2.svg";
import detectiveSVG from "assets/tie.svg";
import mrXSVG from "assets/secret.svg";
import TramSVG from "assets/tram1.svg";

import L, { IconOptions } from "leaflet";

// eslint-disable-next-line react-refresh/only-export-components
export const ICON_OFFSET: [number, number] = [15, 0];

const DEFAULT_SETTINGS: Partial<IconOptions> = {
  iconSize: [30, 30], // size of the icon
  shadowSize: [0, 0], // size of the shadow
  iconAnchor: [15, 15], // point of the icon which will correspond to marker's location
  shadowAnchor: [0, 0], // the same for the shadow
  popupAnchor: [0, 0], // point from which the popup should open relative to the iconAnchor
};

const TrainIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: trainSVG,
});

const DetectiveIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: detectiveSVG,
});

const MrXIcon = L.icon({
  ...DEFAULT_SETTINGS,
  iconUrl: mrXSVG,
});

const PersonIcon = new L.Icon({
  iconUrl: TramSVG,
  iconRetinaUrl: TramSVG,
  iconSize: new L.Point(60, 75),
  className: "leaflet-div-icon",
});

export { TrainIcon, DetectiveIcon, MrXIcon, PersonIcon };
