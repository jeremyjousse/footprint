import { browser } from "$app/environment";
class AppearanceService {
  constructor() {
    // TODO add theme enum dark light
    const DEFAULT_THEME = "dark";

    let theme = DEFAULT_THEME;
    if (browser) {
      theme = localStorage.getItem("color-scheme") || DEFAULT_THEME;

      if (theme == undefined) {
        if (
          window.matchMedia &&
          window.matchMedia("(prefers-color-scheme: dark)").matches
        ) {
          theme = "dark";
        } else {
          theme = "light";
        }
      }

      this.setTheme(theme);

      window
        .matchMedia("(prefers-color-scheme: dark)")
        .addEventListener("change", (event) => {
          theme = event.matches ? "dark" : "light";
          this.setTheme(theme);
        });
    }
  }

  private setTheme(theme: string) {
    if (browser) {
      localStorage.setItem("color-scheme", theme);

      document.documentElement.setAttribute("color-scheme", theme);
      document.documentElement.setAttribute("style", `color-scheme: ${theme}`);
      document.documentElement.setAttribute("class", `h-full ${theme}`);

      const meta = document.head.getElementsByTagName("meta");
      meta.namedItem("color-scheme")?.setAttribute("content", theme);
    }
  }
}

const appearanceService = new AppearanceService();

export default appearanceService;
