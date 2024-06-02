import { Team } from "lib/bindings";

export function TeamCard(props: {
    team: Team;
    selected?: boolean;
    onClick: () => void;
}) {
    const team = props.team;

    const states = {
        selected: "outline outline-solid outline-2 outline-slate-300",
        default: "",
    };

    const state = props.selected ? "selected" : "default";

    return (
        <div
            className={`hover:bg-slate-100 my-1 flex w-full items-center rounded-md transition-all ${states[state]}`}
            onClick={props.onClick}
        >
            <div
                className="m-2 h-10 w-10 rounded"
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
