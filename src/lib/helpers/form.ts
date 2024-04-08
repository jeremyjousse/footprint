import type { z } from "zod";

export const checkboxCheckedUpdate = (
  event: Event,
  field: string,
  form: z.infer<any>
): void => {
  const target = event.target as HTMLFormElement;

  form[field] = target.checked;
};
