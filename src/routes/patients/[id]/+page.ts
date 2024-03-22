import type { Breadcrumb } from "$domain";
import { error } from "@sveltejs/kit";

export const prerender = true;
export const ssr = false;

export type PatientDetailLoadData = {
  id: string;
  breadcrumbs: Breadcrumb[];
};

export const load = async ({ params }): Promise<PatientDetailLoadData> => {
  const breadcrumbs: Breadcrumb[] = [
    {
      link: "/",
      icon: "Home",
    },
    {
      link: "/patients",
      text: "patients.list.title",
      icon: "Patients",
    },
    {
      text: "patients.detail.title",
    },
  ];
  return {
    breadcrumbs: breadcrumbs,
    id: params.id,
  };
};
