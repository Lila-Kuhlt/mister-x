import { BaseButton } from "components/lila/button";
import { TeamCard } from "components/TeamCard";
import { getTeams } from "lib/api";
import { Team } from "lib/bindings";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router-dom";

export function SelectTeam() {
    const [teams, setTeams] = useState<Team[]>([]);
    const [selected, setSelected] = useState<number | undefined>();
    const navigate = useNavigate();
    const { t } = useTranslation();

    useEffect(() => {
        const updateTeams = () => void getTeams().then(setTeams);

        updateTeams();

        const interval = setInterval(updateTeams, 500);
        return () => clearInterval(interval);
    }, []);

    const process = () => {
        if (selected === undefined) return;
        navigate("/game", { state: teams[selected] });
    };

    return (
        <form className="flex h-screen items-center justify-center bg-base">
            <div className="bg-white container flex w-80 flex-col gap-4 p-8">
                <h2 className="text-lg font-semibold">{t("SelectTeam")}</h2>
                <div className="flex flex-col gap-2">
                    {teams.map((team, index) => (
                        <TeamCard
                            key={team.id}
                            team={team}
                            selected={selected === index}
                            onClick={() => setSelected(index)}
                        />
                    ))}
                </div>
                <BaseButton
                    variant="primary"
                    size="md-wide"
                    disabled={selected === undefined}
                    onClick={process}
                >
                    {t("JoinTeam")}
                </BaseButton>
                <Link
                    className="text-muted text-center underline"
                    to="/create"
                >
                    {t("CreateTeam")}
                </Link>
            </div>
        </form>
    );
}