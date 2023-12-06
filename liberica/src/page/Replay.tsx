import { BASE_URLS, ENDPOINTS } from "lib/api"
import { ReplayMessage, ReplayResponse } from "lib/bindings"
import { WebsocketApi } from "lib/websockets"
import { useGameState, useReplayState, useReplayWebsocketStore } from "lib/state"
import { useEffect } from "react"
import { Map } from "page/Map"
import { Button, Dropdown, ProgressBar } from "react-bootstrap"
import { Navbar } from "components/Navbar"

const defaultSpeed = 10

export function Replay() {
  const { ws, setWebsocket } = useReplayWebsocketStore()
  const { setGameState } = useGameState()
  const { setTime, setProgress, setSpeed, setPaused, time, progress, speed, paused } = useReplayState()

  useEffect(() => {
    const socket = new WebsocketApi<ReplayResponse, ReplayMessage>(BASE_URLS.WEBSOCKET + ENDPOINTS.GET_REPLAY, setWebsocket)
      .register((msg) => console.log("Received message", msg))
      .register((resp) => {
        if (resp !== "End") {
          setGameState(JSON.parse(resp.Frame.game_state))
          setTime(resp.Frame.time)
          setProgress(resp.Frame.progress)
        }
      })
    return () => socket.disconnect();
  }, [setGameState, setWebsocket])

  useEffect(() => {
    setSpeed(defaultSpeed)
    if (ws) {
      ws.send({ Speed: defaultSpeed })
    }
  }, [ws])

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

        <label>
          {time} <input
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
        </label>

        <label>
          Speed: <input
            type="number"
            min={1}
            value={speed}
            onChange={(e) => {
              const newSpeed = parseInt(e.target.value)
              if (e.target.validity.valid) {
                setSpeed(newSpeed)
                ws.send({ Speed: newSpeed })
              }
            }}
          />
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
