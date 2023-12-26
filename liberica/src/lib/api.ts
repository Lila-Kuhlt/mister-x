import axios from "axios";
import { Team, Stop, CreateTeam } from "lib/bindings";
import { WebSocketApi } from "./websockets";

const ENDPOINTS = {
  POST_CREATE_TEAM: "/create-team",
  POST_START_GAME: "/start-game",
  GET_TEAMS: "/teams",
  GET_STOPS: "/stops",
  GET_PING: "/ping",
  GET_WS: "/ws",
};

const hostname = location.hostname;
const isDevServer = !import.meta.env.PROD;
const isLocal = ['localhost', '127.0.0.1'].includes(hostname);

/**
 * Get the WebSocket endpoint.
 * E.g. ws://misterx.com/ws
 *
 * Allows insecure websocket connection, if either
 *  - The context itself is not secure (connected via http)
 *  - The current build is not a production build
 *  - The hostname is not localhost (localhost is considedred secure)
 *
 * @returns The websocket endpoint
 */
export function getWebSocketEndpoint(): string {
  const port = isDevServer ? '3000' : location.port;
  const proto = isLocal || isDevServer ? 'ws' : 'wss';

  return `${proto}://${hostname}:${port}`;
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
export function getHTTPEndpoint(): string {
  const port = isDevServer ? '3000' : location.port;
  const proto = isLocal || isDevServer ? 'http' : 'https';

  return `${proto}://${hostname}:${port}/api`;
}

export const BASE_URLS = {
  WEBSOCKET: getWebSocketEndpoint(),
  HTTP: getHTTPEndpoint(),
};

export const AXIOS = axios.create({ baseURL: BASE_URLS.HTTP });

export const postCreateTeam = (team: CreateTeam): Promise<Team> =>
  AXIOS.post(ENDPOINTS.POST_CREATE_TEAM, team);

export const getTeams = (): Promise<Team[]> =>
  AXIOS.get(ENDPOINTS.GET_TEAMS).then((data) => data.data);

export const getStops = (): Promise<Stop[]> =>
  AXIOS.get(ENDPOINTS.GET_STOPS).then((data) => data.data);

export const serverAlive = (): Promise<boolean> =>
  AXIOS.get(ENDPOINTS.GET_PING)
    .then(() => true)
    .catch(() => false);

export const createWebSocketConnection = () =>
  new WebSocketApi(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_WS);
