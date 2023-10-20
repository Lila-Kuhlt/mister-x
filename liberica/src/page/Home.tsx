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
      <div className="w-100 text-center" style={{ maxWidth: "330px" }}>
        <h2 className="p-2">Join a team</h2>
        <ol className="list-group mb-3">
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
            </li>
          ))}
        </ol>
        <Button
          className="w-100"
          variant="primary"
          disabled={!selected}
          onClick={() => selected && setTeam(teams[selected])}
        >
          Join
        </Button>
        <Button
          className="w-100 mt-2 link-secondary"
          variant="link"
          href="/create"
        >
          Create
        </Button>
      </div>
    </div>
  );
}
