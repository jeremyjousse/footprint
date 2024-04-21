import type { ObjectValues } from "$domain/types";

export const ADD_EDIT_ACTIONS = {
  Add: "Add",
  Edit: "Edit",
} as const;

export type AddEditActions = ObjectValues<typeof ADD_EDIT_ACTIONS>;

export const DETAIL_ACTIONS = {
  Add: "Add",
  Edit: "Edit",
  View: "View",
} as const;

export type DetailActions = ObjectValues<typeof DETAIL_ACTIONS>;
