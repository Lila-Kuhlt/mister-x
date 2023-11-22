export type WSHandler<R> = (msg: R) => void;
export type WSDiconnectHandler = () => void;
export type WSErrorHandler = (error?: string | Error | object) => void;

export class WebsocketApi<R, S> {
  lastMessage?: Date;
  connection: WebSocket;
  handlers: WSHandler<R>[];
  errorHandler: WSErrorHandler;
  disconnectHandler: WSDiconnectHandler;

  constructor(endpoint: string, connected: (api: WebsocketApi<R, S>) => void) {
    this.connection = new WebSocket(endpoint);

    this.handlers = [];
    this.errorHandler = (x) => console.error("Websocket Error occured", x);
    this.connection.onerror = this.errorHandler;
    this.disconnectHandler = () => console.log("Closed WS connection");
    this.connection.onopen = () => {
      connected(this);
      console.log("Established WS connection");
    };
    this.connection.onclose = () => {
      this.disconnectHandler();
    };
    this.connection.onmessage = (e) => {
      const json = JSON.parse(e.data as string);
      this.lastMessage = new Date();
      this.handlers.forEach((handler) => handler(json as R));
    };
  }

  public setDisconnectHandler(handler: WSDiconnectHandler): WebsocketApi<R, S> {
    this.disconnectHandler = handler;
    return this;
  }

  public register(handler: WSHandler<R>): WebsocketApi<R, S> {
    this.handlers.push(handler);
    return this;
  }

  public setErrorHandler(handler: WSErrorHandler): WebsocketApi<R, S> {
    this.errorHandler = handler;
    return this;
  }

  public send(msg: S) {
    this.connection.send(JSON.stringify(msg));
  }

  public disconnect() {
    this.connection.close()
  }
}
