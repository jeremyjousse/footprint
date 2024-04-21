import {
  CONSULTATION_LOCATION,
  CONSULTATION_STATUS,
} from "$domain/valueObjects";

import type { RefinementCtx } from "zod";
import { z } from "$i18n/zod";

export const consultationSchema = z.object({
  appointmentDateTime: z
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
    .nullish(),
  consultationType: z.string().min(2), // TODO check from backend
  id: z.string().uuid().nullish(), // TODO only on create
  location: z.nativeEnum(CONSULTATION_LOCATION),
  note: z.string().min(2).nullable(),
  patientId: z.string().uuid(),
  price: z.number().multipleOf(0.01).gt(0),
  status: z.nativeEnum(CONSULTATION_STATUS),
});

export type Consultation = z.infer<typeof consultationSchema>;
