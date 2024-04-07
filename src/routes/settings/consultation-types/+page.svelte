<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import Table from "$components/atoms/dateTable/Table.svelte";
  import TableHead from "$components/atoms/dateTable/TableHead.svelte";
  import TableHeadCell from "$components/atoms/dateTable/TableHeadCell.svelte";
  import type { TableData } from "$domain/valueObjects/DataTable";
  import { consultationTypeService } from "$services";
  import { t } from "$i18n";
  import TableBody from "$components/atoms/dateTable/TableBody.svelte";
  import TableBodyCell from "$components/atoms/dateTable/TableBodyCell.svelte";
  import TableBodyRow from "$components/atoms/dateTable/TableBodyRow.svelte";
  export let data: PatientLoadData;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await consultationTypeService.loadInitData();
  };
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions">
    <Button href="/settings/consultation-types/new" color="primary"
      >{$t("consultationTypes.add.title")}</Button
    >
  </div>
</PageNavbar>
{#await loadTableData() then}
  <Table>
    <TableHead>
      <TableHeadCell
        >{$t("consultationTypes.consultationType.name")}</TableHeadCell
      >
      <TableHeadCell
        >{$t("consultationTypes.consultationType.price")}</TableHeadCell
      >
    </TableHead>
    <TableBody>
      {#each tableData.rows as row}
        <TableBodyRow>
          <TableBodyCell>{row.name}</TableBodyCell>
          <TableBodyCell>{row.price}</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
{/await}
