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
      className={`my-1 flex items-center w-full transition-all rounded-md hover:bg-slate-100 ${states[state]}`}
      onClick={props.onClick}
    >
      <div className="w-10 h-10 rounded m-2" style={{ backgroundColor: team.color }} />
      <div className="flex flex-col justify-center">
        <span className="font-semibold">{team.name}</span>
        <span className="justify-end italic font-normal text-slate-400">
          {team.kind}
        </span>
      </div>
    </div>
  );
}
