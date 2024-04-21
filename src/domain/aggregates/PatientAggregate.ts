import type { ConsultationAggregate } from "./ConsultationAggregate";
import type { Patient } from "$domain/entities";

export class PatientAggregate {
  patient: Patient;
  consultations: ConsultationAggregate[];

  constructor(patient: Patient, consultations: ConsultationAggregate[]) {
    this.patient = patient;
    this.consultations = consultations;
  }
}
