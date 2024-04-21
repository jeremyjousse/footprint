/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        primary: colors.indigo,
        danger: colors.red,
        // Generated with https://uicolors.app/create
        // and https://www.tailwindshades.com/#color=240%2C60.00000000000023%2C99.01960784313727&step-up=8&step-down=11&hue-shift=0&name=white-lilac&base-stop=5&v=1&overrides=e30%3D
        gray_scale: {
          50: "#f6f6f6",
          100: "#e7e7e7",
          200: "#d1d1d1",
          300: "#b0b0b0",
          400: "#888888",
          500: "#6d6d6d",
          600: "#5d5d5d",
          700: "#4f4f4f",
          800: "#454545",
          900: "#3d3d3d",
          950: "#1e1e1e",
        },
      },
    },
  },
  plugins: [],
  darkMode: "selector",
};
