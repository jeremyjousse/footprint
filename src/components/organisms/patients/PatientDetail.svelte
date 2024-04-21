<script lang="ts">
  import Tile from "$components/atoms/Tile.svelte";
  import type { Patient } from "$domain";
  import { t } from "$i18n";

  export let patient: Patient;
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
      <a href="tel:{patient.contactInformation.mobilePhone}"
        >{patient.contactInformation.mobilePhone}</a
      >
    </div>
    <div class="col-span-2">
      <a href="tel:{patient.contactInformation.phone}"
        >{patient.contactInformation.phone}</a
      >
    </div>
  </div>
</Tile>
