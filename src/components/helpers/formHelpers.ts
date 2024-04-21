import type { ColorType } from "$components/types";
import { twMerge } from "tailwind-merge";

export const colorClasses: Record<ColorType, string> = {
  primary: "text-primary-600 ",
  secondary: "text-secondary-600 ",
  gray: "text-gray-900 dark:text-gray-300",
  gray_scale: "text-gray_scale-900 dark:text-gray_scale-300",
  disabled: "text-gray-400 dark:text-gray-500",
};

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
