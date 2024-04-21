import { PAYMENT_METHOD, PAYMENT_STATUS } from "$domain/valueObjects";

import type { RefinementCtx } from "zod";
import { z } from "$i18n/zod";

export const PaymentSchema = z.object({
  consultationId: z.string().uuid(),
  id: z.string().uuid().nullish(),
  payedAt: z
    .string()
    .transform((dateString: string, ctx: RefinementCtx) => {
      if (dateString) {
        const date = new Date(dateString);
        if (!z.date().safeParse(date).success) {
          ctx.addIssue({
            code: z.ZodIssueCode.invalid_date,
          });
        }
      }

      return dateString;
    })
    .nullable(), //TODO
  paymentMethod: z.nativeEnum(PAYMENT_METHOD),
  amount: z.number().multipleOf(0.01).min(1).negative(),
  status: z.nativeEnum(PAYMENT_STATUS),
});

export type Payment = z.infer<typeof PaymentSchema>;
