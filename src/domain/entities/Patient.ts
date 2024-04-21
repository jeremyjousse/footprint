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
    .nullable()
    .optional(),
  // birthdate: z.string().date().nullable(), // TODO wait for Zod new version release
  createdAt: z.date().nullable().optional(),
  contactInformation: z.object({
    email: z.string().email().nullable().default(null).optional(),
    phone: z.string().nullable().default(null).optional(),
    mobilePhone: z.string().nullable().default(null).optional(),
  }),
  diabetic: z.boolean().default(false).optional(),
  id: z.string().uuid(),
  longDurationDisease: z.boolean().default(false).optional(),
  nationalInsuranceNumber: z.string().nullable().default(null).optional(),
  notes: z.string().nullable().default(""),
  personalName: z.object({
    firstName: z.string().min(2).default(""),
    lastName: z.string().min(2).default(""),
  }),
  postalAddress: z.object({
    city: z.string().min(2).nullable().default(null).optional(),
    country: z.string().min(2).nullable().default(null).optional(),
    postalCode: z.string().min(2).nullable().default(null).optional(),
    street: z.string().min(2).nullable().default(null).optional(),
  }),
  updatedAt: z.date().nullable().optional(),
});

export type Patient = z.infer<typeof patientSchema>;

export const patientDummySchema = z.object({
  name: z.string().nullable().optional().default("YOP"),
});

export type PatientDummy = z.infer<typeof patientDummySchema>;
