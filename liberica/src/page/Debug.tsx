import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    Button,
    ButtonSize,
    ButtonVariant,
} from "components/lila/button";
import { TextInput } from "components/lila/input";
import { THEMES, ThemeName, applyTheme } from "lib/theme";
import { useState } from "react";

export function Debug() {
    const sizes = Object.keys(BUTTON_SIZES) as ButtonSize[];
    const variants = Object.keys(BUTTON_VARIANTS) as ButtonVariant[];

    const [text, setText] = useState("Test");
    return (
        <div className="flex h-screen w-dvw flex-col items-center justify-center gap-10 bg-base">
            <TextInput
                placeholder="Enter example text"
                onChange={(e) => setText(e.target.value || "Test")}
            ></TextInput>

            <select
                onChange={(e) => applyTheme(e.target.value as ThemeName, true)}
            >
                {Object.keys(THEMES).map((theme) => (
                    <option key={theme}>{theme}</option>
                ))}
            </select>

            <table className="table-auto border-spacing-2">
                <thead>
                    <tr className="text-left">
                        <th>Variant</th>
                        <th>Size</th>
                        <th>Element</th>
                    </tr>
                </thead>

                <tbody>
                    {variants
                        .flatMap((variant) =>
                            sizes.map((size) => [variant, size]),
                        )
                        .map(([variant, size]) => (
                            <tr key={variant + size} className="text-left">
                                <td className="py-2 pr-4">{variant}</td>
                                <td className="py-2 pr-8">{size}</td>
                                <td className="py-2">
                                    <Button
                                        variant={variant as ButtonVariant}
                                        size={size as ButtonSize}
                                    >
                                        {text}
                                    </Button>
                                </td>
                            </tr>
                        ))}
                </tbody>
            </table>
        </div>
    );
}
