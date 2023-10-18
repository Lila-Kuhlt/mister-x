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
  //let line: Line = { id: 1, name: "asdf", color: "asdf" }
  let testPos = [49.0069, 8.4037]
  //let train: Train = { id: 1, lat: testPos[0], long: testPos[1], line_id: "18", direction: "Istanbul" }
  let mrX: Team = { id: 0, x: 49.012796, y: 8.4031014, name: "Mr X", color: "black" }
  //let player: Player = { id: 2, name: "Detective", x: testPos[0], y: testPos[1] + 0.001, team_id: 1 }
  console.log(props)
  return <SVGMap props={props} />
}
