import type { ColorType } from "$components/types";
import { colorClasses } from "./cssHelpers";
import { twMerge } from "tailwind-merge";

export const inputClass = (
  color: ColorType,
  rounded: boolean,
  tinted: boolean,
  spacing: string,
  extraClass: string
) =>
  twMerge(
    "w-4 h-4 bg-gray-100 border-gray-300 dark:ring-offset-gray-800 focus:ring-2",
    spacing,
    tinted
      ? "dark:bg-gray-600 dark:border-gray-500"
      : "dark:bg-gray-700 dark:border-gray-600",
    rounded && "rounded",
    colorClasses[color],
    extraClass
  );
