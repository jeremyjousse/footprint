<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  export let data: PatientLoadData;
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<a href="/patients/new">Add a patient</a>
<a href="/patients/20">View patient 20</a>
