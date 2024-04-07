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
      text: "settings.title",
      icon: "Settings",
      link: "/settings",
    },
    {
      text: "consultationTypes.list.title",
      icon: "ConsultationType",
      link: "/settings/consultation-types",
    },
    {
      text: "consultationTypes.add.title",
    },
  ];
  return { breadcrumbs };
};
