import { Button } from "components/InputElements";
import { TeamCard } from "components/TeamCard";
import { getTeams } from "lib/api";
import { Team } from "lib/bindings";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router-dom";

export function Home() {
  const [teams, setTeams] = useState<Team[]>([]);
  const [selected, setSelected] = useState<number | undefined>();
  const navigate = useNavigate();
  const { t } = useTranslation();

  useEffect(() => {
    const updateTeams = () => getTeams().then(setTeams);

    updateTeams();
    const interval = setInterval(updateTeams, 500);
    return () => clearInterval(interval);
  }, []);

  const process = async () => {
    if (selected === undefined) {
      return;
    }
    navigate("/game", { state: teams[selected] });
  };

  return (
    <form className="flex items-center justify-center h-screen">
      <div className="container flex flex-col gap-4 p-8 bg-white w-80">
        <h2 className="text-lg font-semibold">{t("SelectTeam")}</h2>
        <div>
          {teams.map((team, index) => (
            <TeamCard
              key={team.id}
              team={team}
              selected={selected === index}
              onClick={() => setSelected(index)}
            />
          ))}
        </div>
        <Button
          disabled={selected === undefined}
          type="button"
          onClick={process}
        >
          {t("JoinTeam")}
        </Button>
        <Link className="text-center underline text-slate-400" to="/create">
          {t("CreateTeam")}
        </Link>
      </div>
    </form>
  );
}
