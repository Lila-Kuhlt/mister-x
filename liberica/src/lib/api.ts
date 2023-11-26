import axios from "axios";
import { Team, Stop, CreateTeam } from "lib/bindings";
import { WebsocketApi } from "./websockets";

const PROTOCOL = {
  WS_SECURE: "wss:",
  WS: "ws:",
  HTTP_SECURE: "https:",
  HTTP: "http:",
};

const ENDPOINTS = {
  POST_CREATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_TEAMS: "/teams",
  GET_STOPS: "/stops",
  GET_PING: "/ping",
  GET_WS: "/ws",
};

/**
 * Get the Websocket endpoint.
 * E.g. ws://misterx.com/ws
 *
 * Allows insecure websocket connection, if either
 *  - The context itself is not secure (connected via http)
 *  - The current build is not a production build
 *  - The hostname is not localhost (localhost is considedred secure)
 *
 * @returns The websocket endpoint
 */
export function getWebsocketEndpoint() {
  const useDevServer = !window.isSecureContext || !import.meta.env.PROD;
  const hostname = window.location.hostname;
  const port = useDevServer ? ":3000" : "";
  const proto =
    useDevServer || hostname === "localhost" ? PROTOCOL.WS : PROTOCOL.WS_SECURE;

  return `${proto}//${hostname}${port}`;
}

/**
 * Get the Http endpoint.
 * E.g. https://misterx.com/api/teams
 *
 * Allows insecure websocket connection, if either
 *  - The context itself is not secure (connected via http)
 *  - The current build is not a production build
 *  - The hostname is not localhost (localhost is considedred secure)
 *
 * @returns The http endpoint
 */
export function getHTTPEndpoint() {
  const useDevServer = !window.isSecureContext || !import.meta.env.PROD;
  const hostname = window.location.hostname;
  const port = useDevServer ? ":3000" : "";
  const proto =
    useDevServer || hostname === "localhost"
      ? PROTOCOL.HTTP
      : PROTOCOL.HTTP_SECURE;

  return `${proto}//${hostname}${port}/api`;
}

export const BASE_URLS = {
  WEBSOCKET: getWebsocketEndpoint(),
  HTTP: getHTTPEndpoint(),
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

export const createWebsocketConnection = () =>
  new WebsocketApi(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS);
