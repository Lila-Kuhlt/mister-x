import train from '../public/tram2.svg';
import detective from '../public/tie.svg';
import mrX from '../public/secret.svg';
import L from 'leaflet';

var TrainIcon = L.icon({
  iconUrl: train,
  shadowUrl: undefined,
  iconSize:     [30, 30], // size of the icon
  shadowSize:   [0, 0], // size of the shadow
  iconAnchor:   [15, 15], // point of the icon which will correspond to marker's location
  shadowAnchor: [0, 0],  // the same for the shadow
  popupAnchor:  [0, 0] // point from which the popup should open relative to the iconAnchor
});

var DetectiveIcon = L.icon({
  iconUrl: detective,
  shadowUrl: undefined,
  iconSize:     [30, 30], // size of the icon
  shadowSize:   [0, 0], // size of the shadow
  iconAnchor:   [15, 30], // point of the icon which will correspond to marker's location
  shadowAnchor: [0, 0],  // the same for the shadow
  popupAnchor:  [0, 0] // point from which the popup should open relative to the iconAnchor
});

var MrXIcon = L.icon({
  iconUrl: mrX,
  shadowUrl: undefined,
  iconSize:     [30, 30], // size of the icon
  shadowSize:   [0, 0], // size of the shadow
  iconAnchor:   [15, 30], // point of the icon which will correspond to marker's location
  shadowAnchor: [0, 0],  // the same for the shadow
  popupAnchor:  [0, 0] // point from which the popup should open relative to the iconAnchor
});

function trainIcon(train: Train) {
  return new L.DivIcon({
    className: 'train-icon',
    html: `<p>${train.line_id}</p>`
  });
}

export { TrainIcon, trainIcon, DetectiveIcon, MrXIcon };
