<script lang="ts">
  import type { PatientLoadData } from "./+page";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import type { TableData } from "$domain/valueObjects/TableData";
  import { patientService } from "$services";
  import { t } from "$i18n";
  import Button from "$components/atoms/Button.svelte";
  export let data: PatientLoadData;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await patientService.loadInitData();
  };
</script>

<PageNavbar>
  <Breadcrumb breadcrumbs={data.breadcrumbs} slot="breadcrumbs" />
</PageNavbar>
<Button href="/settings/bank-accounts">{$t("bankAccounts.list.title")}</Button>
<Button href="/settings/consultation-types"
  >{$t("consultationTypes.list.title")}</Button
>
