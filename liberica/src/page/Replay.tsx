import { useRef, useState } from "react";
import { useInterval } from "use-interval";
import { GameStateContext, Map } from "components/map/Map";
import { Button } from "components/InputElements";
import { HomeButton, Navbar } from "components/Navbar";
import { GameState } from "lib/bindings";
import { clamp } from "lib/util";
import schema from "lib/schema";
import { FaPlay, FaPause } from "react-icons/fa";
import { useTranslation } from "react-i18next";

// configuration
const defaultSpeed = 10;
const ms_per_frame = 50;
const max_replay_size = 100_000_000; // 100 MB

interface Entry {
    time: Date;
    state: GameState;
}

function parseCSV(data: string): Entry[] {
    return data
        .trim()
        .split(/\r?\n/)
        .map((line) => {
            const [time, state] = line.split(", ", 2);
            return {
                time: new Date(time),
                state: schema.GameState.check(JSON.parse(state)),
            };
        });
}

/**
 * Finds the entry whose time is the closest to `time`.
 *
 * Assumes that `state` is not empty and that `time` is in the range of `state`,
 * i.e. not less than `state[0].time.getTime()` and not greater than `state[state.length - 1].time.getTime()`.
 */
function findNearest(state: Entry[], time: number): Entry {
    // binary search
    let low = 0;
    let high = state.length - 1;
    while (low <= high) {
        const mid = (low + high) >>> 1;
        const midEntry = state[mid];
        const midTime = midEntry.time.getTime();
        if (time < midTime) {
            high = mid - 1;
        } else if (time > midTime) {
            low = mid + 1;
        } else {
            return midEntry;
        }
    }

    // the nearest entry is a or b
    const a = state[high];
    const b = state[low];
    if (
        Math.abs(a.time.getTime() - time) <= Math.abs(b.time.getTime() - time)
    ) {
        return a;
    } else {
        return b;
    }
}

type Timestamp = number;
interface ReplayState {
    state: Entry[];
    startTime: Timestamp;
    endTime: Timestamp;
    duration: number;
    position: Timestamp;
    frameTime: number;
}

export function Replay() {
    const { t } = useTranslation();
    const [gs, setGameState] = useState<GameState>({ teams: [], trains: [] });

    // UI state
    const [running, setRunning] = useState(false);
    const [paused, setPaused] = useState(true);
    const [progress, setProgress] = useState(0.0);
    const [time, setTime] = useState<Date>();
    const [speed, setSpeed] = useState(defaultSpeed);

    // replay state (initialized using dummy values to avoid having to check for undefined values)
    const r = useRef<ReplayState>({
        state: [],
        startTime: 0,
        endTime: 0,
        duration: 0,
        position: 0,
        frameTime: 0,
    });

    function reset() {
        setRunning(false);
        setPaused(true);
        setProgress(0.0);
        setTime(undefined);
        setSpeed(defaultSpeed);
    }

    function startReplay(state: Entry[]) {
        setRunning(true);
        setPaused(true);
        setSpeed(defaultSpeed);
        const startTime = state[0].time.getTime();
        const endTime = state[state.length - 1].time.getTime();
        r.current = {
            state,
            startTime,
            endTime,
            duration: endTime - startTime,
            position: startTime,
            frameTime: defaultSpeed * ms_per_frame,
        };
        sendFrame();
    }

    function onProgressChange(newProgress: number) {
        setProgress(newProgress);
        r.current.position =
            r.current.startTime + newProgress * r.current.duration;
        sendFrame();
    }

    function onSpeedChange(newSpeed: number) {
        setSpeed(newSpeed);
        r.current.frameTime = clamp(
            newSpeed * ms_per_frame,
            0.0,
            r.current.duration,
        );
    }

    function sendFrame() {
        const entry = findNearest(r.current.state, r.current.position);
        setTime(new Date(r.current.position));
        setProgress(
            clamp(
                (r.current.position - r.current.startTime) / r.current.duration,
                0.0,
                1.0,
            ),
        );
        setGameState(entry.state);
    }

    useInterval(
        () => {
            if (r.current.position >= r.current.endTime) {
                r.current.position = r.current.endTime;
                sendFrame();
                setPaused(true);
            } else {
                sendFrame();
                r.current.position += r.current.frameTime;
            }
        },
        running && !paused && ms_per_frame,
    );

    return (
        <div className="flex h-max w-max flex-col">
            <GameStateContext.Provider value={gs}>
                <Map />
            </GameStateContext.Provider>

            <Navbar>
                <HomeButton />

                <input
                    type="file"
                    onChange={(e) => {
                        const file = e.target.files?.item(0);
                        if (file) {
                            if (file.size >= max_replay_size) {
                                alert(t("ReplayTooBig"));
                            }
                            file.text().then((data) => {
                                try {
                                    const state = parseCSV(data);
                                    if (state.length > 0) {
                                        startReplay(state);
                                    }
                                } catch (e) {
                                    alert(t("FailedParseReplay"));
                                    console.error(
                                        `failed to parse replay file: ${e}`,
                                    );
                                }
                            });
                        }
                        // invalid replay file
                        reset();
                    }}
                />

                <div className="flex-center flex">
                    <Button
                        onClick={() => setPaused((p) => !p)}
                        disabled={!running}
                    >
                        {paused ? <FaPlay /> : <FaPause />}
                    </Button>

                    <div style={{ width: "8px" }} />

                    <label className="flex flex-col">
                        <input
                            type="range"
                            min={0}
                            max={1}
                            step="any"
                            value={progress}
                            onChange={(e) =>
                                onProgressChange(parseFloat(e.target.value))
                            }
                            disabled={!running}
                        />
                        <div style={{ fontFamily: "monospace" }}>
                            {time && t("time", { time })}
                        </div>
                    </label>
                </div>

                <label>
                    {t("Speed")}:{" "}
                    <input
                        type="number"
                        min={1}
                        value={speed}
                        onChange={(e) => {
                            if (e.target.checkValidity() && e.target.value) {
                                onSpeedChange(parseInt(e.target.value));
                            }
                        }}
                        disabled={!running}
                        style={{ width: "60px" }}
                    />{" "}
                    {"x"}
                </label>
            </Navbar>
        </div>
    );
}
