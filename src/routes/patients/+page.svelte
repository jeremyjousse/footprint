<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import DataTable from "$components/molecules/DataTable.svelte";
  import { t } from "$i18n";
  import { ADD_EDIT_ACTIONS } from "$domain";

  export let data: PatientLoadData;
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
  <div slot="actions">
    <Button
      href={`/patients/edit?action=${ADD_EDIT_ACTIONS.Add}`}
      color="primary">{$t("patients.add.title")}</Button
    >
  </div>
</PageNavbar>

<DataTable data={data.tableData} />
