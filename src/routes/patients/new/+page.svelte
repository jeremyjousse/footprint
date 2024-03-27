<script lang="ts">
  import { t } from "$i18n";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import type { Patient } from "$domain";
  import { invoke } from "@tauri-apps/api/core";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import type { PatientAddLoadData } from "./+page";

  export let data: PatientAddLoadData;

  let patient: Patient = {
    birthdate: null,
    createdAt: null,
    contactInformation: {
      email: null,
      phone: null,
      mobilePhone: null,
    },
    note: "",
    personalName: {
      firstName: "Johnny",
      lastName: "Doe",
    },
    postalAddress: {
      city: null,
      country: null,
      postalCode: null,
      street: null,
    },
    updatedAt: null,
  };
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<PatientForm {patient} />
