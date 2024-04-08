<script lang="ts">
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import Tile from "$components/atoms/Tile.svelte";
  import Title from "$components/atoms/Title.svelte";
  import Modal from "$components/atoms/Modal.svelte";
  import PlusIcon from "$components/atoms/icons/PlusIcon.svelte";
  import DataTable from "$components/molecules/DataTable.svelte";
  import {
    DetailActions,
    type Consultation,
    type Patient,
    type TableData,
  } from "$domain";
  import { t } from "$i18n";
  import { consultationService } from "$services";
  import ConsultationForm from "./ConsultationForm.svelte";

  let consultationModelIsOpen: boolean = false;

  export let patient: Patient;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await consultationService.loadInitData(patient.id);
    tableData.detailAction = handleUpdateConsultationPopup;
  };

  let consultation = consultationService.initNewConsultation(patient.id);
  let consultationFormAction = DetailActions.Add;

  const handleNewConsultationPopup = () => {
    consultation = consultationService.initNewConsultation(patient.id);
    consultationModelIsOpen = true;
    consultationFormAction = DetailActions.Add;
  };

  const handleUpdateConsultationPopup = async (id: string) => {
    consultation = await consultationService.get(id);
    consultationModelIsOpen = true;
    consultationFormAction = DetailActions.Edit;
  };
</script>

<Tile>
  <PageNavbar>
    <Title slot="breadcrumbs">{$t("consultations.list.title")}</Title>
    <div slot="actions">
      <Button on:click={() => handleNewConsultationPopup()}><PlusIcon /></Button
      >
    </div>
  </PageNavbar>

  {#await loadTableData() then}
    <DataTable data={tableData} />
  {/await}
  <Modal bind:open={consultationModelIsOpen}>
    <ConsultationForm
      reloadListAction={loadTableData}
      action={consultationFormAction}
      {consultation}
      bind:modalIsOpen={consultationModelIsOpen}
    />
  </Modal>
</Tile>
