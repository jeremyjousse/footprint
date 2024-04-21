<script lang="ts">
  import AutoComplete from "$components/atoms/form/Autocomplete/Autocomplete.svelte";
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Label from "$components/atoms/form/Label.svelte";
  import FormLine from "$components/molecules/FormLine.svelte";
  import {
    consultationSchema,
    type ConsultationType,
    CONSULTATION_LOCATION,
    DETAIL_ACTIONS,
    type DetailActions,
    PAYMENT_STATUS,
    PAYMENT_METHOD,
    type ConsultationAggregateDto,
  } from "$domain";

  import { t } from "$i18n";
  import { superForm, dateProxy } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";

  import { consultationService, consultationTypeService } from "$services";
  import Button from "$components/atoms/Button.svelte";
  import { onMount } from "svelte";
  import PaymentForm from "../payment/PaymentForm.svelte";
  import PaymentList from "../payment/PaymentList.svelte";
  import { consultationStore } from "$stores/consultationStore";
  import { get } from "svelte/store";

  export let modalIsOpen: boolean;

  export let action: DetailActions = DETAIL_ACTIONS.Add;

  let consultationAggregate: ConsultationAggregateDto;
  export let patient: string;
  export let consultationId: string;

  let payment = undefined;
  let loadConsultation = async () => {
    console.log("loadConsultation");
    if (action == DETAIL_ACTIONS.Add) {
      consultationAggregate = consultationService.initNewConsultation(patient);
    } else {
      consultationAggregate = await consultationService.get(consultationId);
    }
    console.log("loadConsultation2");
  };

  const { form, enhance, errors, message, validateForm } = superForm(
    get(consultationStore),
    {
      validators: zod(consultationSchema),
      dataType: "json",
    }
  );

  const consultationLocationArray = []; //consultationService.locations();

  let consultationTypes: ConsultationType[] = [];
  let consultationTypesArray: any[] = [];

  onMount(async () => {
    // TODO move ?
    console.log("onMount");
    consultationTypes = await consultationTypeService.list();
    console.log("onMount2");
    consultationTypesArray = consultationTypes.map((consultationType) => {
      return {
        [consultationType.id]: consultationType.name as string,
      };
    });
    console.log("onMount3");
  });

  const handleAutoCompleteChange = (field: string, value: any) => {
    switch (field) {
      case "location":
        $form.location =
          CONSULTATION_LOCATION[
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

    const result = await validateForm({ update: true });
    console.log(JSON.stringify(result));

    if (result.valid) {
      if (action == DETAIL_ACTIONS.Add) {
        // $form.id = undefined;
        callSuccess = await consultationService.add($form);
      } else if (action == DETAIL_ACTIONS.Edit) {
        callSuccess = await consultationService.update($form);
      }

      if (callSuccess) {
        reloadListAction();
        modalIsOpen = false;
      }
    }
  };

  const reloadListAction = () => {};

  const handleAddFormPayment = () => {
    payment = {
      consultationId: $form.id,
      amount: $form.price,
      payedAt: new Date().toISOString(),
      paymentMethod: PAYMENT_METHOD.Cash,
      status: PAYMENT_STATUS.Completed,
    };
  };
</script>

{#await loadConsultation() then}
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
        errors={$errors.location}
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
        errors={$errors.consultationType}
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
        >{$t(
          `consultations.${action.toLocaleLowerCase()}.buttonAction`
        )}</Button
      >
    </div>
    {#if consultationAggregate.payments.length > 0}
      <PaymentList
        rows={consultationAggregate.payments}
        reloadPayments={reloadListAction}
      />
    {/if}
    {#if payment != undefined}
      <PaymentForm {payment} editing={true} reloadPayments={reloadListAction} />
    {/if}
    {#if action == DETAIL_ACTIONS.Edit}
      <div class="flex justify-end">
        <Button color="primary" type="button" on:click={handleAddFormPayment}
          >{$t(`payments.add.buttonAction`)}</Button
        >
      </div>
    {/if}
  </form>
{/await}
