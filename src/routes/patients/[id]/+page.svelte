<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Button from "$components/atoms/Button.svelte";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import { patientService } from "$services";
  import type { Patient } from "$domain";

  export let data;
  let patient: Patient;
  const loadPatient = async () => {
    patient = await patientService.get(data.id);
    console.log(patient);
  };
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
{#await loadPatient() then}
  <PatientForm {patient} action="update" />
{/await}
