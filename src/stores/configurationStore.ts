import {
  type Configuration,
  type ApplicationConfiguration,
  defaultApplicationConfiguration,
} from "$domain";

import { writable } from "svelte/store";

export const configurationStore = writable<Configuration>({
  application: defaultApplicationConfiguration(),
});

export const updateApplicationConfiguration = (
  application: ApplicationConfiguration
) => {
  configurationStore.set({
    ...configurationStore,
    application,
  });
};
