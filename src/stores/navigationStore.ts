import { NavigationPages } from "$domain";
import { Store } from "@tauri-apps/plugin-store";
import { goto } from "$app/navigation";
import { writable } from "svelte/store";

type NavigationStore = {
  navigationPage: string;
};

const navigationTauriStore = new Store("navigationStore.json");

export const navigationPage = writable<NavigationPages>(NavigationPages.Home);

const loadNavigationTauriStore = () => {
  navigationTauriStore.get("navigation-page").then((navigationPageLoaded) => {
    if (navigationPageLoaded) {
      switch (navigationPageLoaded) {
        case NavigationPages.Patients: {
          navigationPage.set(NavigationPages.Patients);
          goto("/patients");
          break;
        }
        case NavigationPages.Settings: {
          navigationPage.set(NavigationPages.Settings);
          goto("/settings");
          break;
        }
      }
    }
  });
};

navigationPage.subscribe((value) => {
  if (value) {
    navigationTauriStore.set("navigation-page", value);
    navigationTauriStore.save();
  }
});

loadNavigationTauriStore();
