<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Table from "$components/molecules/Table.svelte";
  import type { TableData } from "$domain/valueObjects/DataTable";
  import { patientService } from "$services";
  export let data: PatientLoadData;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await patientService.loadInitData();
    console.log(tableData);
  };
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<a href="/patients/new">Add a patient</a>
{#await loadTableData() then}
  <Table data={tableData} />
{/await}
