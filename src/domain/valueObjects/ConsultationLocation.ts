import type { ObjectValues } from "$domain/types";

export const CONSULTATION_LOCATION = {
  Clinic: "Clinic",
  Home: "Home",
  Hospital: "Hospital",
} as const;

export type ConsultationLocation = ObjectValues<typeof CONSULTATION_LOCATION>;
