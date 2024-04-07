import { z } from "$i18n/zod";

export const consultationTypeSchema = z.object({
  // id: z.string().uuid().nullable(), // TODO reactivate this with new Zod version
  name: z.string().min(2),
  price: z.number().multipleOf(0.01),
});

export type ConsultationType = z.infer<typeof consultationTypeSchema>;
