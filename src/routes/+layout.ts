export const prerender = true;
export const ssr = false;

import type { Configuration } from "$domain";
import { loadTranslations } from "$i18n";

export const load = async ({ url }) => {
  const { pathname } = url;

  const configuration: Configuration = {
    application: {
      consultationLocations: ["location 1"],
      consultationStatuses: ["status 1"],
    },
  };
  // const configuration = (await invoke('id3_genre_list_command')) as string[];

  return {
    pathname,
    configuration,
  };
};
