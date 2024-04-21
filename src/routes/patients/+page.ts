import type { Breadcrumb } from "$domain";
import type { TableData } from "$domain";
import { patientService } from "$services";

export type PatientLoadData = {
  breadcrumbs: Breadcrumb[];
  tableData: TableData;
};

export const load = async (): Promise<PatientLoadData> => {
  const breadcrumbs: Breadcrumb[] = [
    {
      link: "/",
      icon: "Home",
    },
    {
      text: "patients.list.title",
      icon: "Patient",
    },
  ];
  let tableData: TableData;

  tableData = await patientService.loadInitData();

  return { breadcrumbs, tableData };
};
