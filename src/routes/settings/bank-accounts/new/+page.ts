import type { Breadcrumb } from "domain/valueObjects/Breadcrumb";

export const prerender = true;
export const ssr = false;

export type BankAccountAddLoadData = {
  breadcrumbs: Breadcrumb[];
};

export const load = async (): Promise<BankAccountAddLoadData> => {
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
      link: "/settings/bank-accounts",
    },
    {
      text: "bankAccounts.add.title",
    },
  ];
  return { breadcrumbs };
};
