import { Button } from "./button";
import { TextInput } from "./input.tsx";

export function classes(...inputs: string[]): string {
    return inputs.join(" ");
}

export { Button, TextInput };
