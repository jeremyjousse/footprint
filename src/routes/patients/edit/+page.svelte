<script lang="ts">
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import type { PatientAddEditLoadData } from "./+page";

  export let data: PatientAddEditLoadData;
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions"></div>
</PageNavbar>

<PatientForm patientForm={data.patientForm} action={data.action} id={data.id} />
