<script lang="ts">
  import { superForm } from "sveltekit-superforms/client";
  import { zod } from "sveltekit-superforms/adapters";

  import { t } from "$i18n";
  import { patientSchema, type Patient } from "$domain";
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Button from "$components/atoms/Button.svelte";
  import Checkbox from "$components/atoms/form/Checkbox.svelte";
  import { goto } from "$app/navigation";

  export let patient: Patient;
  export let action: "add" | "update";

  const { form, enhance, errors } = superForm(patient, {
    validators: zod(patientSchema),
    dataType: "json",
  });

  const handleUpdatePatient = async () => {
    if ($form.birthdate == "") {
      $form.birthdate = null; // TODO fix this hack
    }
    await invoke(`patient_${action}_command`, { patient: $form });

    if (action === "add") {
      goto("/patients");
    }
  };

  const handleCheckboxChange = (event: Event, field: string): void => {
    const target = event.target as HTMLFormElement;

    $form[field] = target.checked;
  };
</script>

<form
  method="POST"
  use:enhance
  on:submit|preventDefault={handleUpdatePatient}
  class="mt-8"
>
  {$errors.length}
  <div class="grid grid-cols-5 gap-4">
    <div>{$t("patients.form.name")}</div>
    <div class="col-span-2">
      <FormInput
        type="text"
        name="lastName"
        placeholder={$t("patients.form.lastName")}
        bind:value={$form.personalName.lastName}
        errors={$errors.personalName?.lastName}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        name="firstName"
        placeholder={$t("patients.form.firstName")}
        bind:value={$form.personalName.firstName}
        errors={$errors.personalName?.firstName}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.contactPhone")}</div>
    <div class="col-span-2">
      <FormInput
        name="mobilePhone"
        type="tel"
        placeholder={$t("patients.form.mobilePhone")}
        bind:value={$form.contactInformation.mobilePhone}
        errors={$errors.contactInformation?.mobilePhone}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        type="tel"
        name="phone"
        placeholder={$t("patients.form.phone")}
        bind:value={$form.contactInformation.phone}
        errors={$errors.contactInformation?.phone}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.contactEmail")}</div>
    <div class="col-span-4">
      <FormInput
        name="email"
        type="email"
        placeholder={$t("patients.form.email")}
        bind:value={$form.contactInformation.email}
        errors={$errors.contactInformation?.email}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.postalAddress")}</div>
    <div class="col-span-4">
      <FormInput
        name="street"
        placeholder={$t("patients.form.street")}
        bind:value={$form.postalAddress.street}
        errors={$errors.postalAddress?.street}
      />
    </div>
  </div>
  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>&nbsp</div>
    <div class="col-span-1">
      <FormInput
        name="postalCode"
        placeholder={$t("patients.form.postalCode")}
        bind:value={$form.postalAddress.postalCode}
        errors={$errors.postalAddress?.postalCode}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        name="city"
        placeholder={$t("patients.form.city")}
        bind:value={$form.postalAddress.city}
        errors={$errors.postalAddress?.city}
      />
    </div>
    <div class="col-span-1">
      <FormInput
        name="country"
        placeholder={$t("patients.form.country")}
        bind:value={$form.postalAddress.country}
        errors={$errors.postalAddress?.country}
      />
    </div>
  </div>
  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.health")}</div>
    <div class="col-span-2">
      <Checkbox
        value="1"
        checked={$form.diabetic == 1}
        on:change={(e) => handleCheckboxChange(e, "diabetic")}
        >{$t("patients.form.diabetic")}</Checkbox
      >
    </div>
    <div class="col-span-2">
      <Checkbox
        value="1"
        checked={$form.longDurationDisease == 1}
        on:change={(e) => handleCheckboxChange(e, "longDurationDisease")}
        >{$t("patients.form.longDurationDisease")}</Checkbox
      >
    </div>
  </div>
  <div class="grid grid-cols-5 gap-4 mt-4">
    <div></div>
    <div class="col-span-2">
      <FormInput
        name="country"
        placeholder={$t("patients.form.nationalInsuranceNumber")}
        bind:value={$form.nationalInsuranceNumber}
        errors={$errors.nationalInsuranceNumber}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        name="birthdate"
        type="date"
        placeholder={$t("patients.form.birthdate")}
        bind:value={$form.birthdate}
        errors={$errors.birthdate}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.profession")}</div>
    <div class="col-span-2">
      <FormInput
        name="profession"
        placeholder={$t("patients.form.profession")}
        bind:value={$form.profession}
        errors={$errors.profession}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.notes")}</div>
    <div class="col-span-4">
      <FormInput
        name="notes"
        placeholder={$t("patients.form.notes")}
        bind:value={$form.notes}
        errors={$errors.notes}
      />
    </div>
  </div>

  <div class="flex justify-end">
    <Button color="primary" onclick={() => handleUpdatePatient()}
      >{$t(`patients.${action}.buttonAction`)}</Button
    >
  </div>
</form>
