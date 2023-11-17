import { Team } from "lib/bindings";
import { ColorSwatch } from "./ColorSwatch";

export function TeamCard(props: { team: Team; selected?: boolean }) {
  const team = props.team;
  return (
    <div className="flex items-center w-full my-3 transition-all rounded-md hover:bg-slate-100">
      <ColorSwatch color={team.color} size="12" className="m-2 rounded-md" />
      <div className="flex flex-col justify-center">
        <span className="font-semibold">{team.name}</span>
        <span className="justify-end italic font-normal text-slate-400">
          {team.kind}
        </span>
      </div>
    </div>
  );
}
