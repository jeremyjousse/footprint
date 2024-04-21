<script lang="ts">
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import PatientForm from "$components/organisms/patients/PatientForm.svelte";
  import type { ConsultationAddEditLoadData } from "./+page";

  export let data: ConsultationAddEditLoadData;
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions"></div>
</PageNavbar>
