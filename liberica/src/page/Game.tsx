import { ENDPOINTS } from "lib/api";
import { WebsocketApi } from "lib/websockts";
import { useEffect, useState } from "react";

export function Game() {
  const [ws, setWs] = useState<WebsocketApi | undefined>(undefined);

  useEffect(() => {
    new WebsocketApi(ENDPOINTS.GET_WS, setWs).register(console.log);
  }, []);

  useEffect(() => ws?.send({ Message: "lol" }), [ws]);

  useEffect(() => {
    navigator.geolocation.watchPosition(
      (pos) =>
        pos.coords.altitude &&
        ws?.send({
          Position: { x: pos.coords.altitude, y: pos.coords.longitude },
        })
    );
  }, [ws]);

  return <div></div>;
}
