import { BASE_URLS, ENDPOINTS } from "lib/api"
import { ReplayMessage, ReplayResponse } from "lib/bindings"
import { WebsocketApi } from "lib/websockets"
import { useGameState, useReplayState, useReplayWebsocketStore } from "lib/state"
import { useCallback, useEffect, useState } from "react"
import { Map } from "page/Map"
import { Button } from "react-bootstrap"
import { Navbar } from "components/Navbar"

const defaultSpeed = 10

export function Replay() {
  const { ws, setWebsocket } = useReplayWebsocketStore()
  const { setGameState } = useGameState()
  const { setTime, setProgress, setSpeed, setPaused, time, progress, speed, paused } = useReplayState()
  const [files, setFiles] = useState<string[]>([])

  const resetReplayState = useCallback(() => {
    setSpeed(defaultSpeed)
    setProgress(0.0)
    setPaused(true)
    if (ws) {
      ws.send({ Speed: defaultSpeed })
    }
  }, [ws])

  useEffect(() => {
    const socket = new WebsocketApi<ReplayResponse, ReplayMessage>(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_REPLAY, setWebsocket)
      .register((msg) => console.log("Received message", msg))
    return () => socket.disconnect();
  }, [])
  useEffect(() => {
    ws?.register((resp) => {
      if (resp === "Start") {
        resetReplayState()
      } else if (resp === "End") {
        setPaused(true)
      } else if ("Frame" in resp) {
        setGameState(JSON.parse(resp.Frame.game_state))
        setTime(resp.Frame.time)
        setProgress(resp.Frame.progress)
      } else {
        setFiles(resp.Files)
      }
    })
  }, [resetReplayState, ws])

  useEffect(() => {
    resetReplayState()
  }, [resetReplayState])

  return ws ? (
    <>
      <Map showAll={true}/>
      <Navbar>
        <Button
          onClick={() => {
            window.location.href = "/";
          }}
        >
          <i className="bi bi-house-fill"/>
        </Button>

        <select
          onChange={(e) => {
            ws.send({ Play: e.target.value })
          }}
        >
          {["", ...files].map((file) => (
            <option key={file}>{file}</option>
          ))}
        </select>

        <div className="d-flex">
          <Button
            onClick={() => {
              setPaused(!paused)
              ws.send("Pause")
            }}
          >
            {paused ? (
              <i className="bi bi-play-fill"/>
            ) : (
              <i className="bi bi-pause"/>
            )}
          </Button>

          <div style={{ width: "8px" }}/>

          <label className="d-flex flex-column">
            <input
              type="range"
              min={0}
              max={1}
              step="any"
              value={progress}
              onChange={(e) => {
                const newProgress = parseFloat(e.target.value)
                setProgress(newProgress)
                ws.send({ Goto: newProgress })
              }}
            />
            {time}
          </label>
        </div>

        <label>
          Speed: <input
            type="number"
            min={1}
            value={speed}
            onChange={(e) => {
              if (e.target.validity.valid && e.target.value) {
                const newSpeed = parseInt(e.target.value)
                setSpeed(newSpeed)
                ws.send({ Speed: newSpeed })
              }
            }}
            style={{ width: "60px" }}
          /> x
        </label>
      </Navbar>
    </>
  ) : (
    <div className="d-flex flex-center w-max h-max flex-column">
      <h3>Server is reloading</h3>
      <p>Reload the site in a few seconds</p>
    </div>
  );
}
