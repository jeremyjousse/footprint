import { z } from "$i18n/zod";

export const patientSchema = z.object({
  birthdate: z.coerce.date().nullable(),
  createdAt: z.date().nullable(),
  contactInformation: z.object({
    email: z.string().email({ message: "Invalid email" }).nullable(), // TODO translate
    phone: z.string().nullable(),
    mobilePhone: z.string().nullable(),
  }),
  diabetic: z.boolean().default(false),
  longDurationDisease: z.boolean().default(false),
  nationalInsuranceNumber: z.string().nullable(),
  notes: z.string().min(1, { message: "Notes are required" }), // TODO translate
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
