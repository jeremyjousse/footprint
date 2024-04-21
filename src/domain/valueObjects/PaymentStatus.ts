import type { ObjectValues } from "$domain/types";

export const PAYMENT_STATUS = {
  Canceled: "Canceled",
  Refunded: "Refunded",
  Pending: "Pending",
  Payed: "Payed",
  Completed: "Completed",
} as const;

export type PaymentStatus = ObjectValues<typeof PAYMENT_STATUS>;
