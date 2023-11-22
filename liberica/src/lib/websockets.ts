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
  | { Error: unknown }; // Todo: Better error types

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
    this.connection.onerror = (e) => this.metaHandlers["Error"]?.(e);
    this.connection.onclose = () => this.metaHandlers["Disconnect"]?.();
    this.connection.onopen = () => this.metaHandlers["Connect"]?.();
    this.connection.onmessage = (e) => {
      const res = this.parseMsg(e.data);
      if (res) this.handleMessage(res);
    };
  }

  private parseMsg(msg: string): ServerMessage | undefined {
    try {
      const json = JSON.parse(msg) as ServerMessage;
      return json;
    } catch (e) {
      this.metaHandlers.Error?.(e);
      return undefined;
    }
  }

  private handleMessage(msg: ServerMessage) {
    this.lastMessage = new Date();
    for (const key in msg) {
      const handler = key as Keys<ServerMessage>;
      if (!this.handlers[handler])
        console.warn("No message handler found for message type " + handler);
      this.handlers[handler]?.(msg[key as keyof ServerMessage]);
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
