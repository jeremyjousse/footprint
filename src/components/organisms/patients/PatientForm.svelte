<script lang="ts">
  import { superForm, type SuperValidated } from "sveltekit-superforms/client";
  import { zod } from "sveltekit-superforms/adapters";

  import { t, z } from "$i18n";
  import {
    patientSchema,
    type Patient,
    type DetailActions,
    DETAIL_ACTIONS,
  } from "$domain";
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Button from "$components/atoms/Button.svelte";
  import Checkbox from "$components/atoms/form/Checkbox.svelte";

  import FormLine from "$components/molecules/FormLine.svelte";
  import Label from "$components/atoms/form/Label.svelte";
  import { patientService } from "$services";
  import { goto } from "$app/navigation";

  export let action: DetailActions;
  export let id: string; // TODO UUID type
  export let patientForm: SuperValidated<Patient>;

  const { form, enhance, errors, allErrors } = superForm(patientForm.data, {
    validators: zod(patientSchema),
    dataType: "json",
  });

  const handleUpdatePatient = async (e: SubmitEvent) => {
    e.preventDefault();
    let patientServiceCallResult = undefined;

    // TODO add validation
    // with tooltip

    if (action == DETAIL_ACTIONS.Add) {
      patientServiceCallResult = await patientService.add($form);
    } else if (action == DETAIL_ACTIONS.Edit) {
      patientServiceCallResult = await patientService.update($form);
    }
    if (patientServiceCallResult !== true) {
    } else if (action.toString() === DETAIL_ACTIONS.Add.toString()) {
      // TODO redirect to patient after creation
      await goto("/patients");
    } else if (action.toString() === DETAIL_ACTIONS.Edit.toString()) {
      await goto(`/patients/detail?id=${id}`);
    }
  };

  export const handleCheckboxChange = (
    event: Event,
    field: string,
    form: z.infer<any>
  ): void => {
    const target = event.target as HTMLFormElement;

    form[field] = target.checked;
  };
</script>

<div>
  <form
    use:enhance
    on:submit|preventDefault={handleUpdatePatient}
    method="POST"
  >
    <FormLine>
      <Label slot="label">{$t("patients.form.name")}</Label>
      <FormInput
        type="text"
        name="firstName"
        placeholder={$t("patients.form.lastName")}
        bind:value={$form.personalName.lastName}
        errors={$errors.personalName?.lastName}
        slot="firstFormElement"
      />
      <FormInput
        name="firstName"
        placeholder={$t("patients.form.firstName")}
        bind:value={$form.personalName.firstName}
        errors={$errors.personalName?.firstName}
        slot="secondFormElement"
      />
    </FormLine>
    <FormLine>
      <Label slot="label">{$t("patients.form.contactPhone")}</Label>
      <FormInput
        name="mobilePhone"
        type="tel"
        placeholder={$t("patients.form.mobilePhone")}
        bind:value={$form.contactInformation.mobilePhone}
        errors={$errors.contactInformation?.mobilePhone}
        slot="firstFormElement"
      />
      <FormInput
        type="tel"
        name="phone"
        placeholder={$t("patients.form.phone")}
        bind:value={$form.contactInformation.phone}
        errors={$errors.contactInformation?.phone}
        slot="secondFormElement"
      />
    </FormLine>

    <FormLine>
      <Label slot="label">{$t("patients.form.contactEmail")}</Label>

      <FormInput
        name="email"
        type="email"
        placeholder={$t("patients.form.email")}
        bind:value={$form.contactInformation.email}
        errors={$errors.contactInformation?.email}
        slot="firstFormElement"
      />
    </FormLine>
    <FormLine>
      <Label slot="label">{$t("patients.form.postalAddress")}</Label>

      <FormInput
        name="street"
        placeholder={$t("patients.form.street")}
        bind:value={$form.postalAddress.street}
        errors={$errors.postalAddress?.street}
        slot="firstFormElement"
      />
    </FormLine>
    <FormLine>
      <Label slot="label"></Label>

      <FormInput
        name="postalCode"
        placeholder={$t("patients.form.postalCode")}
        bind:value={$form.postalAddress.postalCode}
        errors={$errors.postalAddress?.postalCode}
        slot="firstFormElement"
      />
      <FormInput
        name="city"
        placeholder={$t("patients.form.city")}
        bind:value={$form.postalAddress.city}
        errors={$errors.postalAddress?.city}
        slot="secondFormElement"
      />

      <FormInput
        name="country"
        placeholder={$t("patients.form.country")}
        bind:value={$form.postalAddress.country}
        errors={$errors.postalAddress?.country}
        slot="thirdFormElement"
      />
    </FormLine>

    <FormLine>
      <Label slot="label">{$t("patients.form.health")}</Label>

      <Checkbox
        value="1"
        checked={$form.diabetic == true}
        on:change={(e) => handleCheckboxChange(e, "diabetic", $form)}
        slot="firstFormElement">{$t("patients.form.diabetic")}</Checkbox
      >
      <Checkbox
        value="1"
        checked={$form.longDurationDisease == true}
        on:change={(e) => handleCheckboxChange(e, "longDurationDisease", $form)}
        slot="secondFormElement"
        >{$t("patients.form.longDurationDisease")}</Checkbox
      >
    </FormLine>

    <FormLine>
      <Label slot="label"></Label>

      <FormInput
        name="country"
        placeholder={$t("patients.form.nationalInsuranceNumber")}
        bind:value={$form.nationalInsuranceNumber}
        errors={$errors.nationalInsuranceNumber}
        slot="firstFormElement"
      />

      <FormInput
        name="birthdate"
        type="date"
        placeholder={$t("patients.form.birthdate")}
        bind:value={$form.birthdate}
        errors={$errors.birthdate}
        slot="secondFormElement"
      />
    </FormLine>

    <FormLine>
      <Label slot="label">{$t("patients.form.notes")}</Label>

      <FormInput
        name="notes"
        placeholder={$t("patients.form.notes")}
        bind:value={$form.notes}
        errors={$errors.notes}
        slot="firstFormElement"
      />
    </FormLine>
    <div class="flex justify-end">
      <Button
        color="primary"
        type="submit"
        disabled={$allErrors.length === 0 ? "" : "disable"}
        >{$t(`patients.${action.toLocaleLowerCase()}.buttonAction`)}</Button
      >
    </div>
  </form>
</div>
