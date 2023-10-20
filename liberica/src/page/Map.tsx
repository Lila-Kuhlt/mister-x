//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import SVGMap from "components/SVGMap";
import { GameState, Train, Team } from "lib/bindings";
export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
}

const testPos = [49.012796, 8.4031014];
const defaultProps = {
  trains: [
    { id: 1, lat: 49.0069, long: 8.4037, line_id: "kvv:abc", line_name: "S100", direction: "Karlsruhe" },
  ],
  teams: [
    {
      id: 1,
      x: testPos[0],
      y: testPos[1] - 0.002,
      name: "Default Team (no connection)",
      color: "blue",
    },
  ],
  mrX: {
    id: 0,
    x: testPos[0],
    y: testPos[1] + 0.002,
    name: "Mr X",
    color: "black"
  },
};

export function Map(props: { gameState: GameState | undefined }) {
  return (
    <SVGMap
      trains={props.gameState?.trains || defaultProps.trains}
      teams={Object.values(props.gameState?.teams ?? defaultProps.teams)}
      mrX={props.gameState?.teams[0] || defaultProps.mrX}
    />
  );
}
