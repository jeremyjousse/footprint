import type { Breadcrumb } from "domain/valueObjects/Breadcrumb";

export type PatientLoadData = {
  breadcrumbs: Breadcrumb[];
};

export const load = async (): Promise<PatientLoadData> => {
  const breadcrumbs: Breadcrumb[] = [
    {
      link: "/",
      icon: "Home",
    },
    {
      text: "settings.title",
      icon: "Settings",
      link: "/settings",
    },
    {
      text: "consultationTypes.list.title",
      icon: "ConsultationType",
    },
  ];
  return { breadcrumbs };
};
