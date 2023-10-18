//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import SVGMap from "components/SVGMap";
import { Train, Team } from "lib/bindings";
export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
}

export function Map(props: MapProps) {
  const testPos = [49.0069, 8.4037];
  const mrX: Team = {
    id: 1,
    name: "Mr. X",
    x: testPos[0],
    y: testPos[1] - 0.001,
    color: "red",
  };
  return <SVGMap trains={props.trains} teams={props.teams} mrX={mrX} />;
}
