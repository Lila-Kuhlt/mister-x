import { useTranslation } from "react-i18next";

export function Admin() {
    const { t } = useTranslation();

    return <>
        <div className="h-10 w-10 m-10 rounded-md bg-cyan-500" />
        <div className="h-10 w-10 m-10 rounded-md bg-surface" />
    </>;
}
