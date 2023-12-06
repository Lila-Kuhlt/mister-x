import { create } from "zustand";
import { persist } from "zustand/middleware";
import { ClientMessage, GameState, ReplayMessage, ReplayResponse, Team, Train } from "lib/bindings";
import { WebsocketApi } from "./websockets";

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

export interface ReplayStateStore {
  time?: string;
  progress?: number;
  speed?: number;
  paused?: boolean;
  setTime: (time?: string) => void;
  setProgress: (progress?: number) => void;
  setSpeed: (speed?: number) => void;
  setPaused: (paused?: boolean) => void;
}

export interface WebsocketStore<R, S> {
  ws?: WebsocketApi<R, S>;
  setWebsocket: (ws?: WebsocketApi<R, S>) => void;
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

export const useReplayState = create<ReplayStateStore>()((set) => ({
  setTime: (time?: string) => set(() => ({ time })),
  setProgress: (progress?: number) => set(() => ({ progress })),
  setSpeed: (speed?: number) => set(() => ({ speed })),
  setPaused: (paused?: boolean) => set(() => ({ paused })),
}));

export const useGameWebsocketStore = useWebsocketStore<GameState, ClientMessage>();
export const useReplayWebsocketStore = useWebsocketStore<ReplayResponse, ReplayMessage>();

function useWebsocketStore<R, S>() {
  return create<WebsocketStore<R, S>>()((set) => ({
    setWebsocket: (ws) => set(() => ({ ws })),
  }));
}
