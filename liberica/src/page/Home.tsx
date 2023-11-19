import { Button } from "components/InputElements";
import { Spinner } from "components/Spinner";
import { TeamCard } from "components/TeamCard";
import { getTeams } from "lib/api";
import { Team } from "lib/bindings";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export function Home() {
  const [teams, setTeams] = useState<Team[]>([]);
  const [selcted, setSelected] = useState<number | undefined>();
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    getTeams().then(setTeams);
  }, []);

  const process = async () => {
    if (selcted === undefined) return;
    navigate("/game");
  };

  return (
    <form className="flex items-center justify-center h-screen">
      <div className="container flex flex-col gap-4 p-8 bg-white w-80">
        <h2 className="text-lg font-semibold">Select a Team</h2>
        <div>
          {teams.map((team, index) => (
            <TeamCard
              team={team}
              selected={selcted === index}
              onClick={() => setSelected(index)}
            />
          ))}
        </div>
        <Button
          disabled={selcted === undefined}
          type="button"
          onClick={() => void process()}
        >
          "Join Team"
        </Button>
        <a className="text-center underline text-slate-400" href="/create">
          Create one instead
        </a>
      </div>
    </form>
  );
}
