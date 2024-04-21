<script lang="ts">
  import Button from "$components/atoms/Button.svelte";
  import PageNavbar from "$components/atoms/PageNavbar.svelte";
  import Tile from "$components/atoms/Tile.svelte";
  import Title from "$components/atoms/Title.svelte";
  import Modal from "$components/atoms/Modal.svelte";
  import PlusIcon from "$components/atoms/icons/PlusIcon.svelte";
  import DataTable from "$components/molecules/DataTable.svelte";
  import {
    DETAIL_ACTIONS,
    type Consultation,
    type ConsultationAggregateDto,
    type Patient,
    type TableData,
  } from "$domain";
  import { t } from "$i18n";
  import { consultationService } from "$services";
  import ConsultationForm from "./ConsultationForm.svelte";

  let consultationModelIsOpen: boolean = false;

  export let patient: Patient;

  let consultationAggregate: ConsultationAggregateDto;

  let tableData: TableData;
  const loadTableData = async () => {
    tableData = await consultationService.loadInitData(patient.id);
    tableData.detailAction = handleUpdateConsultationPopup;
  };

  let consultation = consultationService.initNewConsultation(patient.id);
  let consultationFormAction = DETAIL_ACTIONS.Add;

  const handleNewConsultationPopup = async () => {
    consultationAggregate = consultationService.initNewConsultation(patient.id);
    consultationModelIsOpen = true;
    consultationFormAction = DETAIL_ACTIONS.Add;
  };

  const handleUpdateConsultationPopup = async (id: string) => {
    consultationAggregate = await consultationService.get(id);
    consultationModelIsOpen = true;
    consultationFormAction = DETAIL_ACTIONS.Edit;
  };
</script>

<Tile>
  <PageNavbar>
    <Title slot="breadcrumbs">{$t("consultations.list.title")}</Title>
    <div slot="actions">
      <Button
        href={`/patients/detail/consultations/edit?patientId=${patient.id}`}
        ><PlusIcon /></Button
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
      consultation={consultationAggregate.consultation}
      payments={consultationAggregate.payments}
      bind:modalIsOpen={consultationModelIsOpen}
    />
  </Modal>
</Tile>
