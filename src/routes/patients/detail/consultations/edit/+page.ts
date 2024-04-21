import {
  ADD_EDIT_ACTIONS,
  consultationSchema,
  type AddEditActions,
  type Breadcrumb,
  type Consultation,
  type ConsultationDto,
} from "$domain";

import { superValidate, type SuperValidated } from "sveltekit-superforms";
import { getAddEditActionFromSearchParams } from "$helpers";
import { invoke } from "@tauri-apps/api/core";
import { zod } from "sveltekit-superforms/adapters";

export type ConsultationAddEditLoadData = {
  id: string | null;
  patientId: string;
  breadcrumbs: Breadcrumb[];
  action: AddEditActions;
  patientForm: SuperValidated<Consultation>;
};

export const load = async ({
  url,
}: {
  url: URL;
}): Promise<ConsultationAddEditLoadData> => {
  // todo move this in consultation service
  const id = url.searchParams.get("id");
  const patientId = url.searchParams.get("patientId");
  const action = getAddEditActionFromSearchParams(url.searchParams);
  // TODO throw error if no id and consultation id and action

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
      link: `/patients/detail?id=${patientId}`,
      text: "patients.detail.title",
      icon: "Detail",
    },
    {
      text: `consultations.${action.toLocaleLowerCase()}.title`,
      icon: "Edit",
    },
  ];

  let patientForm: SuperValidated<Consultation>;
  if (action == ADD_EDIT_ACTIONS.Add) {
    const consultationCreateSchema = consultationSchema.extend({
      id: consultationSchema.shape.id.optional(),
    });
    patientForm = await superValidate(zod(consultationCreateSchema));
  } else {
    // TODO throw error if id is null
    const patientDto = await invoke<ConsultationDto>(
      "consultation_detail_command",
      {
        id,
      }
    );
    console.log(patientDto);
    patientForm = await superValidate(patientDto, zod(consultationSchema));
  }

  return {
    breadcrumbs,
    id,
    patientId,
    action,
    patientForm,
  };
};
