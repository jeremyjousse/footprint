import type { Consultation, Payment } from "$domain/entities";

export type ConsultationAggregateDto = {
  consultation: Consultation;
  payments: Payment[];
};
