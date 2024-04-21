import {
  type Breadcrumb,
  type Patient,
  patientSchema,
  type PatientDto,
} from "$domain";
import { getIdFromSearchParamsOrThrowError } from "$helpers";
import { invoke } from "@tauri-apps/api/core";

export type PatientDetailData = {
  id: string;
  breadcrumbs: Breadcrumb[];
  patient: Patient;
};

export const load = async ({
  url,
}: {
  url: URL;
}): Promise<PatientDetailData> => {
  const id = getIdFromSearchParamsOrThrowError(url.searchParams);

  let breadcrumbs: Breadcrumb[] = [
    {
      link: "/",
      icon: "Home",
    },
    {
      link: "/patients",
      text: "patients.list.title",
      icon: "Patient",
    },
    {
      text: "patients.detail.title",
      icon: "Detail",
    },
  ];

  const patientDto = await invoke<PatientDto>("patient_detail_command", { id });

  const patient = patientSchema.parse(patientDto);

  return {
    breadcrumbs: breadcrumbs,
    id: url.searchParams.get("id") || "",
    patient,
  };
};
