import { Team } from "lib/bindings";

export function TeamCard(props: {
    team: Team;
    selected?: boolean;
    onClick: () => void;
}) {
    const team = props.team;

    const states = {
        selected:
            "flex w-full items-center rounded-xl cursor-pointer bg-muted/20",
        default:
            "flex w-full items-center rounded-xl cursor-pointer hover:bg-muted/10",
    };

    const state = props.selected ? "selected" : "default";

    return (
        <div className={states[state]} onClick={props.onClick}>
            <div
                className="m-2 h-10 w-10 rounded-xl"
                style={{ backgroundColor: team.color }}
            />
            <div className="flex flex-col justify-center">
                <span className="font-semibold">{team.name}</span>
                <span className="text-slate-400 justify-end font-normal italic">
                    {team.kind}
                </span>
            </div>
        </div>
    );
}
