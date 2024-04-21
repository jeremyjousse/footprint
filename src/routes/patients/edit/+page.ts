import {
  ADD_EDIT_ACTIONS,
  patientSchema,
  type AddEditActions,
  type Breadcrumb,
  type Patient,
  type PatientDto,
} from "$domain";
import { getAddEditActionFromSearchParams } from "$helpers";
import { invoke } from "@tauri-apps/api/core";
import { superValidate, type SuperValidated } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";

export type PatientAddEditLoadData = {
  id: string | null;
  breadcrumbs: Breadcrumb[];
  action: AddEditActions;
  patientForm: SuperValidated<Patient>;
};

export const load = async ({
  url,
}: {
  url: URL;
}): Promise<PatientAddEditLoadData> => {
  // TODO group action and id and throw error id edit
  const id = url.searchParams.get("id");
  const action = getAddEditActionFromSearchParams(url.searchParams);

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
      link: `/patients/detail?id=${id}`,
      text: "patients.detail.title",
      icon: "Detail",
    },
    {
      text: `patients.${action.toLowerCase()}.title`,
      icon: "Edit",
    },
  ];

  let patientForm: SuperValidated<Patient>;
  if (action == ADD_EDIT_ACTIONS.Add) {
    const patientCreateSchema = patientSchema.extend({
      id: patientSchema.shape.id.optional(),
    });
    patientForm = await superValidate(zod(patientCreateSchema));
  } else {
    // TODO throw error if id is null
    const patientDto = await invoke<PatientDto>("patient_detail_command", {
      id,
    });
    console.log(patientDto);
    patientForm = await superValidate(patientDto, zod(patientSchema));
  }

  return {
    breadcrumbs: breadcrumbs,
    id,
    action,
    patientForm,
  };
};
