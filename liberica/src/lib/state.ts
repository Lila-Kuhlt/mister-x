import { create } from "zustand";
import { persist } from "zustand/middleware";
import { GameState, Team, Train } from "lib/bindings";
import { WebsocketApi } from "./websockts";

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

export interface WebsocketStore {
  ws?: WebsocketApi;
  setWebsocket: (ws?: WebsocketApi) => void;
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

export const useWebsocketStore = create<WebsocketStore>()((set) => ({
  setWebsocket: (ws) => set(() => ({ ws })),
}));
