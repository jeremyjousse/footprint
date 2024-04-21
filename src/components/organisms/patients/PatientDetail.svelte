<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "$components/atoms/Button.svelte";
  import Tile from "$components/atoms/Tile.svelte";
  import { ADD_EDIT_ACTIONS, type Patient } from "$domain";
  import { t } from "$i18n";
  import { patientService } from "$services";

  export let patient: Patient;
  export let id: string; // TODO this is a patient value?

  const deletePatient = async () => {
    const deleted = await patientService.delete(patient.id);
    if (deleted === true) {
      console.log("redirect");
      await goto("/patients");
    }
  };
</script>

<Tile>
  <div class="grid grid-cols-5 gap-4">
    <div class="text-right">{$t("patients.form.name")}</div>
    <div class="col-span-2">{patient.personalName.lastName}</div>
    <div class="col-span-2">{patient.personalName.firstName}</div>
  </div>
  <div class="grid grid-cols-5 gap-4">
    <div class="text-right">{$t("patients.form.contactPhone")}</div>
    <div class="col-span-2">
      <a href="tel:{patient.contactInformation?.mobilePhone}"
        >{patient.contactInformation?.mobilePhone}</a
      >
    </div>
    <div class="col-span-2">
      <a href="tel:{patient.contactInformation?.phone}"
        >{patient.contactInformation?.phone}</a
      >
    </div>
  </div>
  <div class="grid grid-cols-2 gap-4">
    <div class="text-left">
      <!-- TODO do not display if the patient has consultation attached -->
      <Button on:click={deletePatient} color="danger"
        >{$t("patients.delete.buttonAction")}</Button
      >
    </div>
    <div class="text-right">
      <Button
        href={`/patients/edit?action=${ADD_EDIT_ACTIONS.Edit}&id=${id}`}
        color="primary">{$t("patients.edit.buttonAction")}</Button
      >
    </div>
  </div>
</Tile>
