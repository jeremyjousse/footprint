import type { Breadcrumb } from "domain/valueObjects/Breadcrumb";

export const prerender = true;
export const ssr = false;

export type PatientAddLoadData = {
  breadcrumbs: Breadcrumb[];
};

export const load = async (): Promise<PatientAddLoadData> => {
  const breadcrumbs: Breadcrumb[] = [
    {
      link: "/",
      icon: "Home",
    },
    {
      text: "patients.list.title",
      link: "/patients",
      icon: "Patient",
    },
    {
      text: "patients.add.title",
    },
  ];
  return { breadcrumbs };
};
