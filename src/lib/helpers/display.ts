import type { TableColumn } from "$domain";

export const displayTimeFromSeconds = (seconds: number): string => {
  if (isNaN(seconds) || !isFinite(seconds) || seconds < 0) {
    return "00:00";
  }
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins}:${secs}`;
};

export const displayLocaleDateTime = (date: string): string => {
  if (!date) return "";
  const dateObj = new Date(date);
  return dateObj.toLocaleString();
};

export const displayTableRowValue = (
  value: string | number | boolean | null,
  column: TableColumn
): string => {
  if (column.displayHelper) {
    return column.displayHelper(value);
  }

  return value as string;
};
