import { ConsultationLocation, ConsultationStatus } from "$domain/valueObjects";

import type { RefinementCtx } from "zod";
import { z } from "$i18n/zod";

export const consultationSchema = z.object({
  // appointmentDateTime: z.string().datetime().nullable(), // TODO
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
        // console.log(`localISOTime ${localISOTime}`);
        // return date.toISOString().substring(0, 19);
      }

      return dateString;
    })
    .nullable(),
  consultationType: z.string().min(2),
  // id: z.string().uuid().nullable(), // TODO
  location: z.nativeEnum(ConsultationLocation),
  note: z.string().min(2).nullable(),
  patientId: z.string().uuid(),
  price: z.number().multipleOf(0.01),
  status: z.nativeEnum(ConsultationStatus),
});

export type Consultation = z.infer<typeof consultationSchema>;
