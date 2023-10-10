import { ColorPicker, Input,  } from "antd";
import "./CreateTeam.css"
import { useState } from "react";

export function CreateTeam() {
  const [teamName, setTeamName] = useState('');
  const [color, setColor] = useState('#000000');

  return (
    <div className="createTeamBox">
      <div className="createTeamTitle">
        <p>Create your Team</p>
      </div>
      <div className="adjustTeamBox">
        <div className="teamName">
          <label> Team Name:</label>
          <Input placeholder="Teamname" onChange={(e) => setTeamName(e.target.value)}></Input>
        </div>
        <div className="pickTeamColor">
          <label> Pick Team Color:</label>
          <ColorPicker onChange={(_, hex) => setColor(hex)}></ColorPicker>
        </div>
      </div>
    </div>
  );
}
