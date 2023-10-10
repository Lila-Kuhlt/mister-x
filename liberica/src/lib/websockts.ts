import { ClientMessage, ServerMessage } from "lib/bindings";

export type WSHandler = (msg: ServerMessage) => void;
export type WSErrorHandler = (error?: string | Error | object) => void;

export class WebsocketApi {
  connection: WebSocket;
  handlers: WSHandler[];
  errorHandler: WSErrorHandler;

  constructor(endpoint: string) {
    this.connection = new WebSocket(endpoint);

    this.handlers = [];
    this.errorHandler = (x) => console.error("Websocket Error occured", x);
    this.connection.onerror = this.errorHandler;
    this.connection.onopen = () => console.log("Established WS connection");
    this.connection.onclose = () => console.log("Closed WS connection");
    this.connection.onmessage = (e) => {
      if (e.type !== "text") {
        this.errorHandler("Unexpected message type, exptected only text");
        return;
      }

      const json = JSON.parse(e.data as string);
      this.handlers.forEach((handler) => handler(json as ServerMessage));
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
    this.connection.send(JSON.stringify(msg), this.errorHandler);
  }
}
