//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import SVGMap from "components/SVGMap";
import { Train, Team, Stop } from "lib/bindings";
import { useGameState } from "lib/state";
import { getStops } from "lib/api";
import { useEffect, useState } from "react";
export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
}

const testPos = [49.012796, 8.4031014];
const defaultProps = {
  trains: [
    {
      id: 1,
      lat: 49.0069,
      long: 8.4037,
      line_id: "kvv:abc",
      line_name: "S100",
      direction: "Karlsruhe",
    },
  ],
  teams: [
    {
      id: 1,
      long: testPos[0],
      lat: testPos[1] - 0.002,
      name: "Default Team (no connection)",
      color: "blue",
      on_train: null,
    },
  ],
  mrX: {
    id: 0,
    long: testPos[0],
    lat: testPos[1] + 0.002,
    name: "Mr X",
    on_train: null,
    color: "black",
  },
};

export function Map() {
  const { gameState } = useGameState();
  const [stops, setStops] = useState<Stop[]>([]);
  useEffect(() => {
    getStops().then(setStops);
  }, []);

  return (
    <SVGMap
      trains={gameState?.trains || defaultProps.trains}
      teams={Object.values(gameState?.teams ?? defaultProps.teams)}
      mrX={gameState?.teams[0] || defaultProps.mrX}
      stops={stops || []}
    />
  );
}
