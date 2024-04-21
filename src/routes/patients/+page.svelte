<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import DataTable from "$components/molecules/DataTable.svelte";
  import type { TableData } from "$domain/valueObjects/TableData";
  import { patientService } from "$services";
  import { t } from "$i18n";
  import { DetailActions } from "$domain";
  export let data: PatientLoadData;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await patientService.loadInitData();
  };
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions">
    <Button
      href={`/patients/detail?action=${DetailActions.Add}`}
      color="primary">{$t("patients.add.title")}</Button
    >
  </div>
</PageNavbar>

{#await loadTableData() then}
  <DataTable data={tableData} />
{/await}
