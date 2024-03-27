export const prerender = true;
export const ssr = false;

import { loadTranslations } from "$i18n";
// import { localeService } from "$services/localeService";

export const load = async ({ url }) => {
  const { pathname } = url;

  return {
    pathname,
  };
};
