import type { RefinementCtx } from "zod";
import { z } from "$i18n/zod";

export const patientSchema = z.object({
  birthdate: z
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
    .nullable(),
  // birthdate: z.string().date().nullable(), // TODO wait for Zod new version release
  createdAt: z.date().nullable(),
  contactInformation: z.object({
    email: z.string().email().nullable(),
    phone: z.string().nullable(),
    mobilePhone: z.string().nullable(),
  }),
  diabetic: z.boolean().default(false),
  longDurationDisease: z.boolean().default(false),
  nationalInsuranceNumber: z.string().nullable(),
  notes: z.string().nullable(),
  personalName: z.object({
    firstName: z.string().min(2),
    lastName: z.string().min(2),
  }),
  postalAddress: z.object({
    city: z.string().min(2).nullable(),
    country: z.string().min(2).nullable(),
    postalCode: z.string().min(2).nullable(),
    street: z.string().min(2).nullable(),
  }),
  updatedAt: z.date().nullable(),
});

export type Patient = z.infer<typeof patientSchema>;
