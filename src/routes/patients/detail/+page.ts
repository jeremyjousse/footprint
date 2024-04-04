import type { Breadcrumb } from "$domain";
import { error } from "@sveltejs/kit";
import { page } from "$app/stores";

export type PatientDetailLoadData = {
  id: string;
  breadcrumbs: Breadcrumb[];
};

export const load = async ({ url }): Promise<PatientDetailLoadData> => {
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
    id: url.searchParams.get("id") || "",
  };
};
