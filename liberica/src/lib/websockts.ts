import { WebSocket } from "ws";
import { ClientMessage, ServerMessage } from "lib/bindings";

export type WSHandler = (msg: ServerMessage) => void;

export class WebsocketApi {
  connection: WebSocket;
  handlers: WSHandler[];

  constructor(endpoint: string) {
    this.connection = new WebSocket(endpoint);

    this.connection.onmessage;
    this.handlers = [];
  }

  private register(handler: WSHandler): WebsocketApi {
    this.handlers.push(handler);
    return this;
  }

  private send(msg: ClientMessage) {
    this.connection.send(JSON.stringify(msg));
  }
}
