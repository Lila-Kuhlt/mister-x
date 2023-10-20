import axios from "axios";
import { Team } from "lib/bindings";

const HOSTNAME_DEV = window.location.hostname + ":3000";
const HOSTNAME_PROD = window.location.hostname;

export const BASE_URLS = import.meta.env.DEV
  ? {
      WEBSOCKET: `ws://${HOSTNAME_DEV}`,
      FETCH: `http://${HOSTNAME_DEV}/api`,
    }
  : {
      WEBSOCKET: `wss://${HOSTNAME_PROD}`,
      FETCH: `https://${HOSTNAME_PROD}/api`,
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
