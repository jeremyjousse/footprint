import { type ConsultationAggregateDto } from "$domain";
import { writable } from "svelte/store";

const _consultationStore = () => {
  const { set, subscribe, update } = writable<ConsultationAggregateDto>();

  return {
    set,
    subscribe,
    update,
    setConsultation: (consultation: ConsultationAggregateDto) => {
      consultationStore.set(consultation);
    },
  };
};

export const consultationStore = _consultationStore();
