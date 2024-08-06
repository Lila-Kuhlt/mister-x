import { useTranslation } from "react-i18next";
import React from "react";

export function Admin() {
    const { t } = useTranslation();

    return <>{t("AdminPage")}</>;
}
