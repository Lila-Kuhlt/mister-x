//49.0069° N, 8.4037° E
//[49.0069, 8.4037]
import SVGMap from "components/SVGMap";
import { Train, Team } from "lib/bindings";
export interface MapProps {
  trains: Train[];
  teams: Team[];
  mrX?: Team;
}
  
const testPos = [49.0069, 8.4037]
const defaultProps = {
  trains: [ { id: 1, lat: 49.0069, long: 8.4037, line_id: 1, direction: "Karlsruhe" } ],
  teams: [ { id: 1, x: testPos[0], y: testPos[1] - 0.0002, name: "Detective 1", color: "blue" } ],
  mrX: { id: 0, x: 49.012796, y: 8.4031014, name: "Mr X", color: "black" }
};

export function Map(props: MapProps) {
  const testPos = [49.0069, 8.4037];
  let mrX: Team = { id: 0, x: 49.012796, y: 8.4031014, name: "Mr X", color: "black" }

  return (
    <SVGMap 
      trains={ props.trains || defaultProps.trains }
      teams={ props.teams || defaultProps.teams }
      mrX={ props.mrX || defaultProps.mrX }
    />
  )
}
