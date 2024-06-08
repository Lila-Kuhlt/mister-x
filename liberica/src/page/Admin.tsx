import { useTranslation } from "react-i18next";

export function Admin() {
    const { t } = useTranslation();
    return <div>{t("AdminPage")}</div>;
}
