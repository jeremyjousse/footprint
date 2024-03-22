import { derived, get, writable } from "svelte/store";

import { locale } from "@tauri-apps/plugin-os";
import { locales } from "$i18n";

const DEFAULT_LANGUAGE = "en";

export const userLanguage = async (): Promise<string> => {
  const browserLocale = await locale();
  if (!browserLocale) return DEFAULT_LANGUAGE;
  const language = browserLocale.replace("_", "-").split("-")[0];

  if (locales.get().includes(language)) {
    return language;
  } else {
    return DEFAULT_LANGUAGE;
  }
};
