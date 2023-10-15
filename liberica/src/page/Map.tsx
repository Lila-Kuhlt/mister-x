//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import "./Map.css"
import SVGMap from 'components/SVGMap'
import { Train, Team } from 'lib/bindings'
export interface MapProps { trains: Train[], teams: Team[], mrX?: Team }

export function Map(props: MapProps) {
  //let line: Line = { id: 1, name: "asdf", color: "asdf" }
  let testPos = [49.0069, 8.4037]
  //let train: Train = { id: 1, lat: testPos[0], long: testPos[1], line_id: "18", direction: "Istanbul" }
  let mrX: Team = { id: 1, name: "Mr. X", x: testPos[0], y: testPos[1] - 0.001, color: "red" }
  //let player: Player = { id: 2, name: "Detective", x: testPos[0], y: testPos[1] + 0.001, team_id: 1 }
  return <SVGMap trains={props.trains} teams={props.teams} mrX={mrX} />
}
