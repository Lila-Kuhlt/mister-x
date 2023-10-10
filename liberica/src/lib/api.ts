import axios from "axios";
import { Team } from "./bindings";

export const BASE_URLS = {
  WEBSOCKET: "ws://localhost:3000",
  FETCH: "http://localhost:3000",
};

export const ENDPOINTS = {
  POST_CRATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_WS: "ws://localhost:3000/ws",
};

export const AXIOS = axios.create({ baseURL: BASE_URLS.FETCH });

export const postCreateTeam = (name: string, color: string): Promise<void> =>
  AXIOS.post(ENDPOINTS.POST_CRATE_TEAM, { name, color });

export const getTeams = (): Promise<Team[]> =>
  AXIOS.post(ENDPOINTS.POST_CRATE_TEAM).then((data) => JSON.parse(data.data));
