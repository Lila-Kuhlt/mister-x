import { DropDown } from "components/InputElements";
import { Button } from "components/lila/button";
import { TextInput } from "components/lila/input";
import { defaultErrorHandler, postCreateTeam } from "lib/api";
import { TeamKind } from "lib/bindings";
import { FormEvent, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { red, pink, lime, cyan, purple } from "tailwindcss/colors";

export function CreateTeam() {
    const [color, setColor] = useState<string>(purple["500"]);
    const [name, setName] = useState<string>("");
    const [kind, setKind] = useState<TeamKind>("Detective");

    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();
    const { t } = useTranslation();

    const colors = [
        red["500"],
        pink["500"],
        lime["500"],
        cyan["500"],
        purple["500"],
    ];

    const onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        setLoading(true);
        postCreateTeam({ color, name, kind })
            .then(() => navigate("/"))
            .catch(defaultErrorHandler)
            .finally(() => setLoading(false));
    };

    return (
        <div
            className="flex h-screen items-center justify-center transition-colors"
            style={{ backgroundColor: color }}
        >
            <form
                className="container flex w-80 flex-col gap-3 rounded-xl bg-base p-8 shadow-md"
                onSubmit={onSubmit}
            >
                <h2 className="text-xl font-bold">{t("CreateTeam")}</h2>

                <TextInput
                    onChange={(e) => setName(e.target.value)}
                    placeholder="Lila Pause"
                />

                <DropDown<TeamKind>
                    onItemChange={setKind}
                    items={["Detective", "MrX", "Observer"]}
                />

                <div className="flex justify-between gap-3">
                    {colors.map((color) => (
                        <div
                            className="h-10 w-10 rounded-md"
                            style={{ backgroundColor: color }}
                            key={color}
                            onClick={() => setColor(color)}
                        />
                    ))}
                </div>

                <Button variant="primary" size="md-wide" disabled={loading}>
                    {loading ? (
                        <div className="h-4 w-4 animate-spin rounded-full border-4 border-dashed"></div>
                    ) : (
                        <>{t("CreateTeam")}</>
                    )}
                </Button>
                <Button
                    variant="muted"
                    size="md-wide"
                    disabled={loading}
                    onClick={(e) => {
                        e.preventDefault();
                        navigate("/");
                    }}
                >
                    {t("Cancel")}
                </Button>
            </form>
        </div>
    );
}
