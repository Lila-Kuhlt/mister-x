import {
  Button,
  DropDown,
  ColorSwatchSelect,
  TextInput,
} from "components/InputElements";
import { postCreateTeam } from "lib/api";
import { TeamKind } from "lib/bindings";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";

export function CreateTeam() {
  const [color, setColor] = useState<string>("red-500");
  const [name, setName] = useState<string>("");
  const [kind, setKind] = useState<TeamKind>("Detective");

  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  // WARNING! These are preloaded colors (see tailwind configuraton).
  // If the colors are changed, they have to be "compiled" by tailwind,
  // thus they need to be added in the tailwind configuration
  const colors = ["red-500", "pink-500", "lime-500", "cyan-500", "purple-500"];

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!color || !name || !kind) return;

    console.log("Sending request  ");
    setLoading(true);
    postCreateTeam({ color, name, kind })
      .then(() => setLoading(false))
      .then(() => navigate("/"));
  };

  return (
    <div
      className={`flex items-center justify-center h-screen bg-${color} transition-colors`}
    >
      <form
        className="container p-8 bg-white shadow-md rounded-xl w-80"
        onSubmit={onSubmit}
      >
        <h2 className="text-2xl font-bold">Create team</h2>

        <TextInput onTextChange={setName} trim="all" />
        <DropDown<TeamKind>
          onItemChange={setKind}
          items={["Detective", "MrX", "Observer"]}
        />
        <div className="flex justify-between gap-3 mt-3">
          {colors.map((color) => (
            <div
              className={`w-10 h-10 bg-${color} rounded-md`}
              key={color}
              onClick={() => setColor(color)}
            />
          ))}
        </div>

        <Button disabled={loading}>
          {loading ? (
            <div className="w-4 h-4 border-4 border-dashed rounded-full animate-spin dark:border-white"></div>
          ) : (
            <>Create Team</>
          )}
        </Button>
      </form>
    </div>
  );
}
