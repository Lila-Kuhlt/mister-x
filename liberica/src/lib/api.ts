import axios from "axios";
import { Team, Stop } from "lib/bindings";

const HOSTNAME_DEV = window.location.hostname;
const HOSTNAME_PROD = window.location.host;

export const BASE_URLS = import.meta.env.DEV
  ? {
    WEBSOCKET: `ws://${HOSTNAME_DEV}:3000`,
    FETCH: `http://${HOSTNAME_DEV}:3000/api`,
  }
  : {
    WEBSOCKET: `wss://${HOSTNAME_PROD}`,
    FETCH: `https://${HOSTNAME_PROD}/api`,
  };

export const ENDPOINTS = {
  POST_CRATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_TEAMS: "/teams",
  GET_STOPS: "/stops",
  GET_PING: "/ping",
  GET_WS: "/ws",
};

export const AXIOS = axios.create({ baseURL: BASE_URLS.FETCH });

export const postCreateTeam = (name: string, color: string): Promise<void> =>
  AXIOS.post(ENDPOINTS.POST_CRATE_TEAM, { name, color });

export const getTeams = (): Promise<Team[]> =>
  AXIOS.get(ENDPOINTS.GET_TEAMS).then((data) => data.data);

export const getStops = (): Promise<Stop[]> =>
  AXIOS.get(ENDPOINTS.GET_STOPS).then((data) => data.data);

export const serverAlive = (): Promise<boolean> =>
  AXIOS.get(ENDPOINTS.GET_PING).then(() => true).catch(() => false);
