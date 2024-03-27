import i18next from "i18next";
import translationsEn from "zod-i18n-map/locales/en/zod.json";
import translationsFr from "zod-i18n-map/locales/fr/zod.json";
import { z } from "zod";
import { zodI18nMap } from "zod-i18n-map";

i18next.init({
  lng: "fr",
  resources: {
    en: { zod: translationsEn },
    fr: { zod: translationsFr },
  },
});
z.setErrorMap(zodI18nMap);

// export configured zod instance
export { z };
