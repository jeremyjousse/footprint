import type { RefinementCtx } from "zod";
import { z } from "$i18n/zod";

export const patientSchema = z.object({
  // birthdate: z
  //   .string()
  //   .transform((dateString: string) => {
  //     const dateParsed = Date.parse(dateString);
  //     if (isNaN(dateParsed)) {
  //       console.log("paq dateParsed");
  //       // TODO error
  //     }
  //     console.log(dateParsed);

  //     const date = new Date(dateString);
  //     console.log(date.toISOString().split("T")[0]);

  //     return date.toISOString().split("T")[0];
  //   })
  //   .nullable(),
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
      // TODO nullable
      return dateString;
    })
    .nullable(),
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
