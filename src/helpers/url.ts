import { ADD_EDIT_ACTIONS, type AddEditActions } from "$domain";

export const getAddEditActionFromSearchParams = (
  searchParams: URLSearchParams
): AddEditActions => {
  let action = searchParams.get("action");
  if (action && Object.keys(ADD_EDIT_ACTIONS).includes(action)) {
    return ADD_EDIT_ACTIONS[action as keyof typeof ADD_EDIT_ACTIONS];
  }
  return ADD_EDIT_ACTIONS.Add;
};

export const getIdFromSearchParamsOrThrowError = (
  searchParams: URLSearchParams
): string => {
  const id = searchParams.get("id");
  if (!id) {
    throw new Error("Unable to get id parameter from the URL search params");
  }
  return id;
};
