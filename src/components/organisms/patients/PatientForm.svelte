<script lang="ts">
  import { superForm } from "sveltekit-superforms/client";
  import { zod } from "sveltekit-superforms/adapters";

  import { t } from "$i18n";
  import { patientSchema, type Patient } from "$domain";
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import FormButton from "$components/atoms/form/FormButton.svelte";

  export let patient: Patient;

  const { form, enhance, errors } = superForm(patient, {
    validators: zod(patientSchema),
    dataType: "json",
  });

  const handleCreatePatient = async () => {
    await invoke("patient_add_command", { patient: $form });
  };
</script>

<form method="POST" use:enhance class="mt-8">
  <div class="grid grid-cols-5 gap-4">
    <div>{$t("patients.form.name")}</div>
    <div class="col-span-2">
      <FormInput
        name="firstName"
        placeholder={$t("patients.form.firstName")}
        bind:value={$form.personalName.firstName}
        errors={$errors.personalName?.firstName}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        type="text"
        name="lastName"
        placeholder={$t("patients.form.lastName")}
        bind:value={$form.personalName.lastName}
        errors={$errors.personalName?.lastName}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("patients.form.contactPhone")}</div>
    <div class="col-span-2">
      <FormInput
        name="mobilePhone"
        placeholder={$t("patients.form.mobilePhone")}
        bind:value={$form.contactInformation.mobilePhone}
        errors={$errors.contactInformation?.mobilePhone}
      />
    </div>
    <div class="col-span-2">
      <FormInput
        type="text"
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

  <div class="flex justify-end">
    <FormButton onclick={() => handleCreatePatient()}
      >{$t("patients.add.title")}</FormButton
    >
  </div>
</form>
