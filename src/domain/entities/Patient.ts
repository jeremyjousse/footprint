import { z } from "$i18n/zod";

export const patientSchema = z.object({
  birthdate: z.number().nullable(),
  createdAt: z.date().nullable(),
  contactInformation: z.object({
    email: z.string().email({ message: "Invalid email" }).nullable(),
    phone: z.string().nullable(),
    mobilePhone: z.string().nullable(),
  }),
  note: z.string().min(1, { message: "Note is required" }),
  personalName: z.object({
    firstName: z.string().min(2),
    lastName: z.string().min(2),
  }),
  postalAddress: z.object({
    city: z.string().min(2),
    country: z.string().min(2),
    postalCode: z.string().min(2),
    street: z.string().min(2),
  }),
  updatedAt: z.date().nullable(),
});

export type Patient = z.infer<typeof patientSchema>;
