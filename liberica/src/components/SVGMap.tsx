import { Train, Player} from 'lib/bindings'

export default function SVGMap(props: {trains: Train[], players: Player[]}) {
  const stations = [
    {x: 100, y: 100, name: "Hamburg"},
  ];
  return <div>
      <svg>
        {/* Background map */}
        <use href="public/map.svg" />

        {/* Dynamic train stations */}
        {stations.map(station => (
          <rect key={station.name} x={station.x} y={station.y} width="10" height="10" fill="yellow" />
        ))}

        {/* Dynamic train cars */}
        {props.trains.map(train => (
          <rect key={train.id} x={train.x} y={train.y} width="10" height="10" fill="yellow" />
        ))}

        {/* Dynamic Players */}
        {props.players.map(player => (
          <rect key={player.id} x={player.x} y={player.y} width="10" height="10" fill="red" />
        ))}
      </svg>
    </div>
}
