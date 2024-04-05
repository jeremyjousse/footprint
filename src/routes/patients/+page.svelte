<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import Table from "$components/molecules/Table.svelte";
  import type { TableData } from "$domain/valueObjects/DataTable";
  import { patientService } from "$services";
  import { t } from "$i18n";
  export let data: PatientLoadData;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await patientService.loadInitData();
  };
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions">
    <Button href="/patients/new" color="primary"
      >{$t("patients.add.title")}</Button
    >
  </div>
</PageNavbar>

{#await loadTableData() then}
  <Table data={tableData} />
{/await}
