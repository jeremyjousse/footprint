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
      text: "bankAccounts.list.title",
      icon: "ConsultationType",
    },
  ];
  return { breadcrumbs };
};
