import { TeamCard } from "components/TeamCard";
import { getTeams } from "lib/api";
import { Team } from "lib/bindings";
import { useEffect, useState } from "react";

export function Home() {
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    getTeams().then(setTeams);
  }, []);

  return (
    <div className="flex items-center justify-center h-screen ">
      <div className="container p-8 bg-white w-80">
        <h2 className="font-semibold">Join a Team</h2>

        {teams.map((team) => (
          <TeamCard team={team} />
        ))}
      </div>
    </div>
  );
}
