import { z } from "zod";

export const BankAccountSchema = z.object({
  id: z.string().uuid().optional(),
  bankName: z.string().min(2).max(255),
  accountNumber: z.string().min(2).max(255),
  bankChequeDepositNumber: z.number().max(4294967295),
});

export type BankAccount = z.infer<typeof BankAccountSchema>;
