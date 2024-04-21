import type { ConsultationAggregateDto } from "./ConsultationAggregateDto";
import type { PatientDto } from "./PatientDto";

export type PatientAggregateDto = {
  patient?: PatientDto;
  consultations: ConsultationAggregateDto[];
};
