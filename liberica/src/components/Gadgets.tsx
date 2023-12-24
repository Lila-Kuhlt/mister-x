import { useEffect, useState } from "react";
import { Button } from "./InputElements";
import { getStops } from "lib/api";
import { Stop } from "lib/bindings";

export function Gadgets() {
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
                <label>
                    <input type="radio" name="gadget" value={undefined} />
                    Stop
                    <select>
                        {stops.map((stop) => (
                            <option key={stop.id} value={stop.id}>
                                {stop.name}
                            </option>
                        ))}
                    </select>
                </label>
                <label>
                    <input type="radio" name="gadget" value="OutOfOrder" />
                    Out of Order
                </label>
                <label>
                    <input type="radio" name="gadget" value="Shackles" />
                    Fessel
                </label>
                <Button>Use</Button>
            </div>
        </form>
    );
}
