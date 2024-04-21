import type { ColorType } from "$components/types";

export const colorClasses: Record<ColorType, string> = {
  primary: "text-primary-600 ",
  secondary: "text-secondary-600 ",
  gray: "text-gray-900 dark:text-gray-300",
  gray_scale: "text-gray_scale-900 dark:text-gray_scale-300",
  disabled: "text-gray-400 dark:text-gray-500",
};
