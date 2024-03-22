class AppearanceService {
  constructor() {
    let theme = localStorage.getItem("color-scheme");

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

  private setTheme(theme: string) {
    localStorage.setItem("color-scheme", theme);
    document.documentElement.setAttribute("color-scheme", theme);
    document.documentElement.setAttribute("style", `color-scheme: ${theme}`);
    document.documentElement.setAttribute("class", `h-full ${theme}`);
    const meta = document.head.getElementsByTagName("meta");
    meta.namedItem("color-scheme")?.setAttribute("content", theme);
  }
}

const appearanceService = new AppearanceService();

export default appearanceService;
