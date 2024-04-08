import { type Breadcrumb, DetailActions } from "$domain";

export type PatientLoadData = {
  id: string;
  breadcrumbs: Breadcrumb[];
  action: DetailActions;
};

export const load = async ({ url }): Promise<PatientLoadData> => {
  // TODO update the breadcrumbs according to the action

  const action =
    DetailActions[
      url.searchParams.get("action") as keyof typeof DetailActions
    ] || DetailActions.Add;
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
    },
  ];

  if (action == DetailActions.Edit) {
    breadcrumbs[2].link = `/patients/detail?id=${url.searchParams.get("id")}&action=${DetailActions.View}`;
    breadcrumbs.push({
      text: "patients.edit.title",
    });
  }
  return {
    breadcrumbs: breadcrumbs,
    id: url.searchParams.get("id") || "",
    action,
  };
};
