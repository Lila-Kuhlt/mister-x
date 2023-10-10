import { ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useEffect, useState } from "react";

export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);

  useEffect(() => {
    new WebsocketApi(ENDPOINTS.GET_WS, setWs).register(console.log);
  }, []);

  useEffect(() => ws?.send({ Message: "lol" }), [ws]);

  return <div></div>;
}
