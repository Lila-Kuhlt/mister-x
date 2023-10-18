import { ColorPicker, Input } from "antd";
import Style from "style/CreateTeam.module.css";
import { useState } from "react";

export function CreateTeam() {
  const [teamName, setTeamName] = useState("");
  const [color, setColor] = useState("#000000");

  return (
    <div className={Style.createTeamBox}>
      <div className={Style.createTeamTitle}>
        <p>Create your Team</p>
      </div>
      <div className={Style.adjustTeamBox}>
        <div className={Style.teamName}>
          <label> Team Name:</label>
          <Input
            placeholder="Teamname"
            onChange={(e) => setTeamName(e.target.value)}
          ></Input>
        </div>
        <div className={Style.pickTeamColor}>
          <label> Pick Team Color:</label>
          <ColorPicker onChange={(_, hex) => setColor(hex)}></ColorPicker>
        </div>
      </div>
    </div>
  );
}
