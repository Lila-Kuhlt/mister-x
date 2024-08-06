import { Team } from "lib/bindings";
import React from "react";

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
            className={`my-1 flex w-full items-center rounded-md transition-all hover:bg-slate-100 ${states[state]}`}
            onClick={props.onClick}
        >
            <div
                className="m-2 h-10 w-10 rounded"
                style={{ backgroundColor: team.color }}
            />
            <div className="flex flex-col justify-center">
                <span className="font-semibold">{team.name}</span>
                <span className="justify-end font-normal italic text-slate-400">
                    {team.kind}
                </span>
            </div>
        </div>
    );
}
