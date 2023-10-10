import SVGMap from 'components/SVGMap'
import { Train, Line } from 'lib/bindings'

export default function Map() {
  let line: Line = { id: 1, name: "asdf", color: "asdf" }
  let train: Train = { id: 1, x: 1, y: 1, line_id: line, direction: "asdf" }
  return <SVGMap trains={[ train ]} players={[]} />
}
