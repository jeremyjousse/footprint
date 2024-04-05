<script lang="ts">
  import { t } from "$i18n";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import type { Patient } from "$domain";
  import { invoke } from "@tauri-apps/api/core";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import type { PatientAddLoadData } from "./+page";
  import { patientService } from "$services";

  export let data: PatientAddLoadData;
  let patient = patientService.initNewPatient();
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<PatientForm {patient} action="add" />
