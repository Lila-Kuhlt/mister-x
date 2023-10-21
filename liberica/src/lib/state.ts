import { create } from "zustand";
import { persist } from "zustand/middleware";
import { GameState, Team, Train } from "lib/bindings";

export interface TeamStore {
  team?: Team;
  setTeam: (team?: Team) => void;
}

export interface GameStateStore {
  gameState?: GameState;
  embarkedTrain?: Train;
  setGameState: (gameState?: GameState) => void;
  setEmbarkedTrain: (train?: Train) => void;
}

export const useTeamStore = create<TeamStore>()(
  persist(
    (set) => ({
      team: undefined,
      setTeam: (team?: Team) => set(() => ({ team })),
    }),
    { name: "team-store" }
  )
);

export const useGameState = create<GameStateStore>()((set) => ({
  setGameState: (state?: GameState) => set(() => ({ gameState: state })),
  setEmbarkedTrain: (embarkedTrain?: Train) => set(() => ({ embarkedTrain })),
}));
