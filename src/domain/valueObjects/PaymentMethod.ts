import type { ObjectValues } from "$domain/types";

export const PAYMENT_METHOD = {
  BankTransfer: "BankTransfer",
  BankCheque: "BankCheque",
  Card: "Card",
  Cash: "Cash",
} as const;

export type PaymentMethod = ObjectValues<typeof PAYMENT_METHOD>;
