import { create } from "zustand";
import { persist } from "zustand/middleware";
import { Team } from "lib/bindings";

export interface TeamStore {
  team?: Team;
  setTeam: (team: Team) => void;
}

export const useTeamStore = create<TeamStore>()(
  persist(
    (set) => ({
      team: undefined,
      setTeam: (team: Team) => set({ team }),
    }),
    { name: "team-store" }
  )
);
