<script lang="ts">
  import Button from "$components/atoms/Button.svelte";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import { patientService } from "$services";
  import { type Patient, DetailActions } from "$domain";

  import ConsultationList from "$components/organisms/consultations/ConsultationList.svelte";
  import PatientDetail from "$components/organisms/patients/PatientDetail.svelte";
  import { setContext } from "svelte";

  export let data;
  let patient: Patient;
  let action: Exclude<DetailActions, DetailActions.View>;
  const loadPatient = async () => {
    setContext("pageActions", data.action);
    if (DetailActions.Add == data.action) {
      patient = patientService.initNewPatient();
      action = DetailActions.Add;
    } else {
      action = DetailActions.Edit;
      patient = await patientService.get(data.id);
    }
  };
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions"></div>
</PageNavbar>
{#await loadPatient() then}
  <PatientForm {patient} {action} />
  {#if DetailActions.Add != data.action}
    <ConsultationList {patient} />
  {/if}
{/await}
