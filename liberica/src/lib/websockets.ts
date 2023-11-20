import { ClientMessage, ServerMessage } from "lib/bindings";

export type Keys<T> = T extends T ? keyof T : never;
export type Concrete<T, K extends Keys<T>> = T extends { [P in K]: infer V }
  ? V
  : never;

export type WSDiconnectHandler = () => void;
export type WSErrorHandler = (error?: string | Error | object) => void;
export type WSHandler<T, K extends Keys<T>> = (msg: Concrete<T, K>) => void;
export type WSHandlerMap<M extends object> = {
  [K in Keys<M>]?: WSHandler<M, K>;
};

export type WSEvent =
  | { Disconnect: void }
  | { Connect: void }
  | { Error: void };

export class WebsocketApi {
  public lastMessage?: Date;

  private connection!: WebSocket;
  private endpoint!: string;

  private handlers: WSHandlerMap<ServerMessage> = {};
  private metaHandlers: WSHandlerMap<WSEvent> = {};

  constructor(endpoint: string) {
    this.connect(endpoint);
  }

  public reconnect(force: boolean = false) {
    // Don't try to reconnect if there is a connection already
    if (this.connection?.readyState === this.connection.OPEN && force) return;

    this?.disconnect();
    this.connect(this.endpoint);
  }

  public connect(endpoint: string) {
    console.log("Connecting to ", endpoint);
    this.endpoint = endpoint;
    this.connection = new WebSocket(endpoint);
    this.connection.onerror = () => this.metaHandlers["Error"]?.();
    this.connection.onclose = () => this.metaHandlers["Disconnect"]?.();
    this.connection.onopen = () => this.metaHandlers["Connect"]?.();
    this.connection.onmessage = (e) =>
      this.handleMessage(JSON.parse(e.data as string) as ServerMessage);
  }

  private handleMessage(msg: ServerMessage) {
    this.lastMessage = new Date();
    for (const key in Object.keys(msg)) {
      this.handlers[key as Keys<ServerMessage>]?.(
        msg[key as keyof ServerMessage]
      );
    }
  }

  public register<T extends Keys<ServerMessage>>(
    type: T,
    handler: WSHandlerMap<ServerMessage>[T]
  ): WebsocketApi {
    this.handlers[type] = handler;
    return this;
  }

  public registerEvent<T extends Keys<WSEvent>>(
    type: T,
    handler: WSHandlerMap<WSEvent>[T]
  ) {
    this.metaHandlers[type] = handler;
    return this;
  }

  public send(msg: ClientMessage) {
    this.connection.send(JSON.stringify(msg));
  }

  public disconnect() {
    this.connection.close();
  }
}
