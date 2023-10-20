import { useState } from "react";
import { Button, Form, InputGroup } from "react-bootstrap";
import Colorful from "@uiw/react-color-colorful";

export function CreateTeam() {
  const [teamName, setTeamName] = useState("");
  const [color, setColor] = useState(`#9900EF`);

  return (
    <div
      className="d-flex flex-column justify-content-center align-items-center h-max"
      style={{ backgroundColor: color }}
    >
      <div className="bg-white p-3 rounded shadow-lg">
        <Form.Control placeholder="Team name" />
        <div className="d-flex flex-column justify-content-center align-items-center p-2">
          <Colorful
            className="m-2"
            disableAlpha
            onChange={(data) => setColor(data.hex)}
            color={color}
          />
        </div>
        <div className="d-grid gap-2">
          <Button variant="primary">Create</Button>
        </div>
      </div>
    </div>
  );
}
