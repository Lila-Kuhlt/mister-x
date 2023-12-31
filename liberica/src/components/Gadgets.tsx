import { PropsWithChildren, useEffect, useState } from "react";
import { Button } from "./InputElements";
import { getStops } from "lib/api";
import { DetectiveGadget, MrXGadget, Stop } from "lib/bindings";

function Gadget(
    props: PropsWithChildren<{ value: string; onSelect?: () => void }>,
) {
    return (
        <label>
            <input
                type="radio"
                name="gadget"
                value={undefined}
                onChange={props.onSelect}
            />
            {props.children}
        </label>
    );
}

export function MrXGadgets() {
    const [gadget, setGadget] = useState<MrXGadget>();
    const [stops, setStops] = useState<Stop[]>([]);
    useEffect(() => {
        getStops().then(setStops);
    }, []);

    return (
        <form
            onSubmit={(e) => {
                e.preventDefault();
                alert(e.target);
            }}
        >
            <div className="flex flex-col">
                <Gadget value="AlternativeFacts">
                    Alternative Fakten:{" "}
                    <select>
                        {stops.map((stop) => (
                            <option key={stop.id} value={stop.id}>
                                {stop.name}
                            </option>
                        ))}
                    </select>
                </Gadget>
                <Gadget value="MidJourney">
                    Midjourney: <input type="file" accept="image/*" />
                </Gadget>
                <Gadget value="NotFound" onSelect={() => setGadget("NotFound")}>
                    404
                </Gadget>
                <Gadget value="Teleport" onSelect={() => setGadget("Teleport")}>
                    Teleport
                </Gadget>
                <Gadget value="Shifter" onSelect={() => setGadget("Shifter")}>
                    Shifter
                </Gadget>
                <Button disabled={gadget === undefined}>Use</Button>
            </div>
        </form>
    );
}

export function DetectiveGadgets() {
    const [gadget, setGadget] = useState<DetectiveGadget>();
    const [stop, setStop] = useState<string>("");
    const [stops, setStops] = useState<Stop[]>([]);
    useEffect(() => {
        getStops().then(setStops);
    }, []);

    return (
        <form
            onSubmit={(e) => {
                e.preventDefault();
                alert(gadget);
            }}
        >
            <div className="flex flex-col">
                <Gadget
                    value="Stop"
                    onSelect={() => {
                        if (stop) {
                            setGadget({ Stop: { stop_id: stop } });
                        } else {
                            setGadget(undefined);
                        }
                    }}
                >
                    Stop:{" "}
                    <select onChange={(e) => setStop(e.target.value)}>
                        {stops.map((stop) => (
                            <option key={stop.id}>{stop.name}</option>
                        ))}
                    </select>
                </Gadget>
                <Gadget
                    value="OutOfOrder"
                    onSelect={() => setGadget("OutOfOrder")}
                >
                    Out of Order
                </Gadget>
                <Gadget value="Shackles" onSelect={() => setGadget("Shackles")}>
                    Fessel
                </Gadget>
                <Button disabled={gadget === undefined}>Use</Button>
            </div>
        </form>
    );
}
