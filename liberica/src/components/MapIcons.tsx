import trainSVG from "assets/tram2.svg";
import detectiveSVG from "assets/tie.svg";
import mrXSVG from "assets/secret.svg";

import L, { IconOptions } from "leaflet";

const DEFAULT_TRAIN_SETTINGS: Partial<IconOptions> = {
    iconSize: [30, 30],
    tooltipAnchor: [15, 0],
};

const DEFAULT_TEAM_SETTINGS: Partial<IconOptions> = {
    iconSize: [30, 30],
    tooltipAnchor: [0, -15],
};

export const TrainIcon = L.icon({
    ...DEFAULT_TRAIN_SETTINGS,
    iconUrl: trainSVG,
});

export const DetectiveIcon = L.icon({
    ...DEFAULT_TEAM_SETTINGS,
    iconUrl: detectiveSVG,
});

export const MrXIcon = L.icon({
    ...DEFAULT_TEAM_SETTINGS,
    iconUrl: mrXSVG,
});
