import { ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useState } from "react";

export function Game() {
  const [ws, setWs] = useState(
    new WebsocketApi(ENDPOINTS.GET_WS).register(console.log)
  );

  return <div></div>;
}
