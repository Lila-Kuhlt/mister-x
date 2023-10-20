import { getTeams } from "lib/api";
import { Team } from "lib/bindings";
import { useTeamStore } from "lib/state";
import { opt } from "lib/util";
import { useEffect, useState } from "react";
import { Button } from "react-bootstrap";

export function Home() {
  const [teams, setTeams] = useState<Team[]>([]);
  const [selected, setSelected] = useState<number | undefined>(undefined);
  const TS = useTeamStore();

  useEffect(() => void getTeams().then(setTeams), []);

  const setTeam = (team: Team) => {
    TS.setTeam(team);
    window.location.href = "/game";
  };

  return (
    <div className="d-flex w-max h-max flex-center flex-column">
      <ol className="list-group list-group-numbered w-75">
        {teams.map((team) => (
          <li
            key={team.id}
            className={
              "list-group-item list-group-item-action d-flex justify-content-between align-items-start " +
              opt(selected === team.id, "active")
            }
            onClick={() => setSelected(team.id)}
          >
            <div className="ms-2 me-auto">
              <div className="fw-bold">{team.name}</div>
            </div>
            <span className="badge bg-primary rounded-pill">14</span>
          </li>
        ))}
      </ol>
      <div className="d-grid col-6 mx-auto m-4">
        <Button
          variant="primary m-2"
          disabled={!selected}
          onClick={() => selected && setTeam(teams[selected])}
        >
          Join
        </Button>
      </div>
    </div>
  );
}
