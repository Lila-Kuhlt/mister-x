import { Button } from "./button";
import { TextInput } from "./input.tsx";

// eslint-disable-next-line react-refresh/only-export-components
export function classes(...inputs: string[]): string {
    return inputs.join(" ");
}

export { Button, TextInput };
