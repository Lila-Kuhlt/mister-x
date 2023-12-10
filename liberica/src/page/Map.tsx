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

export function Map(props: { showAll?: boolean }) {
  const { gameState } = useGameState();
  const [stops, setStops] = useState<Stop[]>([]);
  useEffect(() => {
    // ignore failures
    getStops().then(setStops).catch(() => {});
  }, []);

  return (
    <SVGMap
      trains={gameState?.trains ?? []}
      teams={gameState?.teams ?? []}
      stops={stops}
      showAll={props.showAll ?? false}
    />
  );
}
