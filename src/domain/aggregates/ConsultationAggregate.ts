import type { Consultation, Payment } from "$domain/entities";

export class ConsultationAggregate {
  consultation: Consultation;
  payments: Payment[];
  constructor(consultation: Consultation, payments: Payment[]) {
    this.consultation = consultation;
    this.payments = payments;
  }
}
