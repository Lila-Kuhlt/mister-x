import { PropsWithChildren, useEffect, useState } from "react";
import { Button } from "./InputElements";
import { getStops } from "lib/api";
import { Stop, Team } from "lib/bindings";

function Gadget(props: PropsWithChildren<{ value: string }>) {
    return (
        <label>
            <input type="radio" name="gadget" value={undefined} />
            {props.children}
        </label>
    );
}

export function Gadgets({ team }: { team: Team }) {
    const [stops, setStops] = useState<Stop[]>([]);
    useEffect(() => {
        getStops().then(setStops);
    }, []);

    switch (team.kind) {
        case "MrX":
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
                        <Gadget value="NotFound">404</Gadget>
                        <Gadget value="Teleport">Teleport</Gadget>
                        <Gadget value="Shifter">Shifter</Gadget>
                        <Button>Use</Button>
                    </div>
                </form>
            );
        case "Detective":
            return (
                <form
                    onSubmit={(e) => {
                        e.preventDefault();
                        alert(e.target);
                    }}
                >
                    <div className="flex flex-col">
                        <Gadget value="Stop">
                            Stop:{" "}
                            <select>
                                {stops.map((stop) => (
                                    <option key={stop.id} value={stop.id}>
                                        {stop.name}
                                    </option>
                                ))}
                            </select>
                        </Gadget>
                        <Gadget value="OutOfOrder">Out of Order</Gadget>
                        <Gadget value="Shackles">Fessel</Gadget>
                        <Button>Use</Button>
                    </div>
                </form>
            );
        default:
            return <></>;
    }
}
