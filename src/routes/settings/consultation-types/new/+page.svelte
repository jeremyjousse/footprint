<script lang="ts">
  import { t } from "$i18n";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import ConsultationTypeForm from "$components/organisms/consultationTypes/ConsultationTypeForm.svelte";
  import type { PatientAddLoadData } from "./+page";
  import { consultationTypeService } from "$services";

  export let data: PatientAddLoadData;
  let consultationType = consultationTypeService.initNewConsultationType();
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<ConsultationTypeForm {consultationType} action="add" />
