<script lang="ts">
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Label from "$components/atoms/form/Label.svelte";
  import FormLine from "$components/molecules/FormLine.svelte";
  import {
    consultationSchema,
    type Consultation,
    ConsultationLocation,
    DetailActions,
    type ConsultationType,
  } from "$domain";
  import { t } from "$i18n";
  import SuperDebug, { superForm, dateProxy } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";
  import AutoComplete from "$components/atoms/form/Autocomplete/Autocomplete.svelte";
  import { consultationService, consultationTypeService } from "$services";
  import Button from "$components/atoms/Button.svelte";
  import { onMount } from "svelte";

  export let modalIsOpen: boolean;
  export let consultation: Consultation;
  export let reloadListAction: () => void;

  export let action: DetailActions = DetailActions.Add;

  const { form, enhance, errors } = superForm(consultation, {
    validators: zod(consultationSchema),
    dataType: "json",
  });

  let appointmentDateTime = dateProxy(form, "appointmentDateTime", {
    format: "datetime-local",
  });

  const consultationLocationArray = Object.keys(ConsultationLocation).map(
    (value) => {
      return {
        [value]: $t(`consultations.form.locations.${value}`) as string,
      };
    }
  );

  //TODO
  let consultationTypes: ConsultationType[] = [];
  let consultationTypesArray: any[] = [];

  onMount(async () => {
    consultationTypes = await consultationTypeService.list();
    consultationTypesArray = consultationTypes.map((consultationType) => {
      return {
        [consultationType.id]: consultationType.name as string,
      };
    });
  });

  const handleAutoCompleteChange = (field: string, value: any) => {
    switch (field) {
      case "location":
        $form.location =
          ConsultationLocation[
            Object.keys(value)[0] as keyof typeof ConsultationLocation
          ]; //TODO
        break;
      case "consultationType":
        const selectedConsultationType = Object.keys(value)[0]; //TODO
        let consultationType = consultationTypes.filter(
          (consultationType) => consultationType.id == selectedConsultationType
        )[0]; // TODO secure this and move to the consultationTypeService

        $form.price = consultationType.price;
        $form.consultationType = consultationType.name;
        break;
    }
  };

  const handleUpdateConsultation = async () => {
    let callSuccess = false;

    // TODO move
    if ($form.appointmentDateTime) {
      const date = new Date($form.appointmentDateTime);
      $form.appointmentDateTime = new Date(
        date.getTime() - date.getTimezoneOffset() * 60000
      )
        .toISOString()
        .slice(0, -5);
    }

    if (action == DetailActions.Add) {
      // $form.id = undefined;
      callSuccess = await consultationService.add($form);
    } else if (action == DetailActions.Edit) {
      callSuccess = await consultationService.update($form);
    }

    if (callSuccess) {
      reloadListAction();
      modalIsOpen = false;
    }
  };
</script>

<form
  method="POST"
  use:enhance
  on:submit|preventDefault={handleUpdateConsultation}
>
  <FormLine>
    <Label slot="label">{$t("consultations.form.date")}</Label>
    <FormInput
      name="appointmentDateTime"
      type="datetime-local"
      placeholder={$t("consultations.form.appointmentDateTime")}
      bind:value={$form.appointmentDateTime}
      errors={$errors.appointmentDateTime}
      slot="firstFormElement"
    />
    <AutoComplete
      name="location"
      options={consultationLocationArray}
      onSubmit={handleAutoCompleteChange}
      placeholder={$t("consultations.form.location")}
      isSelectRequired={true}
      slot="secondFormElement"
      value={$form.location}
    />
  </FormLine>
  <FormLine>
    <Label slot="label">{$t("consultations.form.price")}</Label>
    <AutoComplete
      name="consultationType"
      options={consultationTypesArray}
      onSubmit={handleAutoCompleteChange}
      placeholder={$t("consultations.form.consultationType")}
      isSelectRequired={true}
      slot="firstFormElement"
      value={$form.consultationType}
    />
    <FormInput
      name="price"
      type="number"
      min="1"
      step="any"
      placeholder={$t("consultations.form.price")}
      bind:value={$form.price}
      errors={$errors.price}
      slot="secondFormElement"
    />
  </FormLine>
  <FormLine>
    <Label slot="label">{$t("consultations.form.note")}</Label>
    <FormInput
      name="note"
      type="text"
      min="1"
      step="any"
      placeholder={$t("consultations.form.note")}
      bind:value={$form.note}
      errors={$errors.note}
      slot="firstFormElement"
    />
  </FormLine>
  <div class="flex justify-end">
    <Button color="primary" type="button" on:click={handleUpdateConsultation}
      >{$t(`consultations.${action.toLocaleLowerCase()}.buttonAction`)}</Button
    >
  </div>
</form>
