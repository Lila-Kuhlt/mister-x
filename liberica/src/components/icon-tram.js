import L from 'leaflet';

const iconPerson = new L.Icon({
    iconUrl: require('../public/tram1.svg'),
    iconRetinaUrl: require('../public/tram1.svg'),
    iconAnchor: null,
    popupAnchor: null,
    shadowUrl: null,
    shadowSize: null,
    shadowAnchor: null,
    iconSize: new L.Point(60, 75),
    className: 'leaflet-div-icon'
});

export { iconPerson };
