import type { ObjectValues } from "$domain/types";

export const CONSULTATION_STATUS = {
  Cancelled: "Cancelled",
  Done: "Done",
  Pending: "Pending",
};

export type ConsultationStatus = ObjectValues<typeof CONSULTATION_STATUS>;
