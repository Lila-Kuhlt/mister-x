//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import "./Map.css"
import SVGMap from 'components/SVGMap'
import { Train, Player} from 'lib/bindings'

export default function Map() {
  //let line: Line = { id: 1, name: "asdf", color: "asdf" }
  let testPos = [49.0069, 8.4037]
  let train: Train = { id: 1, lat: testPos[0], long: testPos[1], line_id: "18", direction: "Istanbul" }
  let mrX: Player = { id: 1, name: "Mr. X", x: testPos[0], y: testPos[1] + 0.0005, team_id: 0}
  let player: Player = { id: 2, name: "Detective", x: testPos[0], y: testPos[1] + 0.001, team_id: 1}
  return <div>
    <SVGMap trains={[train]} players={[player]} mrX={mrX} />
   </div>
}
