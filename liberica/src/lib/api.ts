import axios from "axios";
import { Team, Stop, CreateTeam } from "lib/bindings";

const PROTOCOLS = {
  WS_SECURE: "wss:",
  WS: "ws:",
  HTTP_SECURE: "https:",
  HTTP: "http:",
};

export function getWebsocketEndpoint() {
  const useDevServer = !window.isSecureContext || !import.meta.env.PROD;
  const proto = useDevServer ? PROTOCOLS.WS : PROTOCOLS.WS_SECURE;
  const hostname = window.location.hostname;
  const port = useDevServer ? ":3000" : "";
  return `${proto}//${hostname}${port}`;
}

export function getHTTPEndpoint() {
  const useDevServer = !window.isSecureContext || !import.meta.env.PROD;
  const proto = useDevServer ? PROTOCOLS.HTTP : PROTOCOLS.HTTP_SECURE;
  const hostname = window.location.hostname;
  const port = useDevServer ? ":3000" : "";
  return `${proto}//${hostname}${port}/api`;
}

export const BASE_URLS = {
  WEBSOCKET: getWebsocketEndpoint(),
  HTTP: getHTTPEndpoint(),
};

export const ENDPOINTS = {
  POST_CREATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_TEAMS: "/teams",
  GET_STOPS: "/stops",
  GET_PING: "/ping",
  GET_WS: "/ws",
};

export const AXIOS = axios.create({ baseURL: BASE_URLS.HTTP });

export const postCreateTeam = (team: CreateTeam): Promise<void> =>
  AXIOS.post(ENDPOINTS.POST_CREATE_TEAM, team);

export const getTeams = (): Promise<Team[]> =>
  AXIOS.get(ENDPOINTS.GET_TEAMS).then((data) => data.data);

export const getStops = (): Promise<Stop[]> =>
  AXIOS.get(ENDPOINTS.GET_STOPS).then((data) => data.data);

export const serverAlive = (): Promise<boolean> =>
  AXIOS.get(ENDPOINTS.GET_PING)
    .then(() => true)
    .catch(() => false);
