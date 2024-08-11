import { useTranslation } from "react-i18next";

export function Admin() {
    const { t } = useTranslation();

    return <>{t("AdminPage")}</>;
}
