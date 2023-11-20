import { ClientMessage } from "lib/bindings";

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

export class WebsocketApi {
  lastMessage?: Date;
  connection: WebSocket;
  handlers: WSHandlerMap<ClientMessage> = {};
  errorHandler: WSErrorHandler;
  disconnectHandler: WSDiconnectHandler;

  constructor(endpoint: string, connected: (api: WebsocketApi) => void) {
    this.connection = new WebSocket(endpoint);

    this.errorHandler = (x) => console.error("Websocket Error occured", x);
    this.connection.onerror = this.errorHandler;
    this.disconnectHandler = () => console.log("Closed WS connection");
    this.connection.onopen = () => {
      connected(this);
      console.log("Established WS connection");
    };
    this.connection.onclose = () => this.disconnectHandler();
    this.connection.onmessage = (e) =>
      this.handleMessage(JSON.parse(e.data as string) as ClientMessage);
  }

  private handleMessage(msg: ClientMessage) {
    this.lastMessage = new Date();
    Object.keys(msg).forEach((key) =>
      this.handlers[key as Keys<ClientMessage>]?.(
        msg[key as keyof ClientMessage]
      )
    );
  }

  public disconnect() {
    this.connection.close();
  }

  public setDisconnectHandler(handler: WSDiconnectHandler): WebsocketApi {
    this.disconnectHandler = handler;
    return this;
  }

  public register<T extends Keys<ClientMessage>>(
    type: T,
    handler: WSHandlerMap<ClientMessage>[T]
  ): WebsocketApi {
    this.handlers[type] = handler;
    return this;
  }

  public setErrorHandler(handler: WSErrorHandler): WebsocketApi {
    this.errorHandler = handler;
    return this;
  }

  public send(msg: ClientMessage) {
    this.connection.send(JSON.stringify(msg));
  }

  public disconnect() {
    this.connection.close()
  }
}
