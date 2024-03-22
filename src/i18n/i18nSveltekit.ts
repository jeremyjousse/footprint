import i18n from "sveltekit-i18n";

const config = {
  loaders: [
    {
      locale: "en",
      key: "",
      loader: async () => (await import("./translations/en.json")).default,
    },
    {
      locale: "fr",
      key: "",
      loader: async () => (await import("./translations/fr.json")).default,
    },
  ],
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(
  config
);
