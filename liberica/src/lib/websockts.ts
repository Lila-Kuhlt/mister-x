import { ClientMessage, GameState } from "lib/bindings";

export type WSHandler = (msg: GameState) => void;
export type WSErrorHandler = (error?: string | Error | object) => void;

export class WebsocketApi {
  lastMessage?: Date;
  connection: WebSocket;
  handlers: WSHandler[];
  errorHandler: WSErrorHandler;

  constructor(endpoint: string, connected: (api: WebsocketApi) => void) {
    this.connection = new WebSocket(endpoint);

    this.handlers = [];
    this.errorHandler = (x) => console.error("Websocket Error occured", x);
    this.connection.onerror = this.errorHandler;
    this.connection.onopen = () => {
      connected(this);
      console.log("Established WS connection");
    };
    this.connection.onclose = () => {
      console.log("Closed WS connection");
    };
    this.connection.onmessage = (e) => {
      const json = JSON.parse(e.data as string);
      this.lastMessage = new Date();
      this.handlers.forEach((handler) => handler(json as GameState));
    };
  }

  public register(handler: WSHandler): WebsocketApi {
    this.handlers.push(handler);
    return this;
  }

  public setErrorHandler(handler: WSErrorHandler): WebsocketApi {
    this.errorHandler = handler;
    return this;
  }

  public send(msg: ClientMessage) {
    this.connection.send(JSON.stringify(msg));
  }
}
