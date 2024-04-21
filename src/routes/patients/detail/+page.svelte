<script lang="ts">
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import ConsultationList from "$components/organisms/consultations/ConsultationList.svelte";
  import PatientDetail from "$components/organisms/patients/PatientDetail.svelte";
  import { type PatientDetailData } from "./+page";

  export let data: PatientDetailData; // TODO
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions"></div>
</PageNavbar>

<PatientDetail patient={data.patient} id={data.id} />

<ConsultationList patient={data.patient} />
