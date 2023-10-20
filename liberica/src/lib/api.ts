import axios from "axios";
import { Team } from "lib/bindings";

export const BASE_URLS = {
  WEBSOCKET: "ws://localhost:3000",
  FETCH: "http://localhost:3000/api",
};

export const ENDPOINTS = {
  POST_CRATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_TEAMS: "/teams",
  GET_WS: "ws://localhost:3000/ws",
};

export const AXIOS = axios.create({ baseURL: BASE_URLS.FETCH });

export const postCreateTeam = (name: string, color: string): Promise<void> =>
  AXIOS.post(ENDPOINTS.POST_CRATE_TEAM, { name, color });

export const getTeams = (): Promise<Team[]> =>
  AXIOS.get(ENDPOINTS.GET_TEAMS).then((data) => data.data);
