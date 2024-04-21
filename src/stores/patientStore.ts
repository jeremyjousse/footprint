import type { PatientDto } from "$domain";
import { writable } from "svelte/store";

const _patientStore = () => {
  const { set, subscribe, update } = writable<PatientDto | undefined>();

  return {
    set,
    subscribe,
    update,
    setPatient: (patient: PatientDto) => {
      patientStore.set(patient);
    },
  };
};

export const patientStore = _patientStore();
