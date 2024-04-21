<script lang="ts">
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Label from "$components/atoms/form/Label.svelte";
  import FormLine from "$components/molecules/FormLine.svelte";
  import AutoComplete from "$components/atoms/form/Autocomplete/Autocomplete.svelte";

  import {
    DETAIL_ACTIONS,
    PAYMENT_METHOD,
    type PaymentMethod,
    type DetailActions,
    type Payment,
    PaymentSchema,
  } from "$domain";
  import { t } from "$i18n";
  import { paymentService } from "$services";
  import { superForm } from "sveltekit-superforms";
  import Button from "$components/atoms/Button.svelte";
  import { zod } from "sveltekit-superforms/adapters";
  import TableBodyRow from "$components/atoms/dateTable/TableBodyRow.svelte";
  import TableBodyCell from "$components/atoms/dateTable/TableBodyCell.svelte";

  export let payment: Payment;
  export let action: DetailActions = DETAIL_ACTIONS.Add;
  export let editing: boolean;
  export let reloadPayments: () => void;

  const { form, errors, message, constraints, enhance } = superForm(payment, {
    validators: zod(PaymentSchema),
    dataType: "form",
  });

  const handleAutoCompleteChange = (field: string, value: any) => {
    switch (field) {
      case "paymentMethod":
        $form.paymentMethod = value;
        break;
    }
  };

  const handleUpdatePayment = async () => {
    let callSuccess = false;

    // TODO move
    if ($form.payedAt) {
      const date = new Date($form.payedAt);
      $form.payedAt = new Date(
        date.getTime() - date.getTimezoneOffset() * 60000
      )
        .toISOString()
        .slice(0, -5);
    }

    if (action == DETAIL_ACTIONS.Add) {
      // $form.id = null;
      callSuccess = await paymentService.add($form);
    } else if (action == DETAIL_ACTIONS.Edit) {
      callSuccess = await paymentService.update($form);
      editing = false;
      // callSuccess = await paymentService.update($form);
    }

    reloadPayments();
  };

  // consultationId: $form.id,

  //     payedAt: new Date().toISOString(),
  //     status: PAYMENT_STATUS.Completed,
</script>

<TableBodyRow>
  <TableBodyCell colspan="4">
    <form
      method="POST"
      use:enhance
      on:submit|preventDefault={handleUpdatePayment}
      style="display: block;"
    >
      <FormLine>
        <Label slot="label">{$t("consultations.form.payedAt")}</Label>
        <FormInput
          name="payedAt"
          type="datetime-local"
          placeholder={$t("consultations.form.payedAt")}
          bind:value={$form.payedAt}
          errors={$errors.payedAt}
          slot="firstFormElement"
        />
      </FormLine>
      <FormLine>
        <Label slot="label">{$t("consultations.form.paymentMethod")}</Label>
        <AutoComplete
          name="paymentMethod"
          options={Object.values(PAYMENT_METHOD)}
          onSubmit={handleAutoCompleteChange}
          placeholder={$t("consultations.form.paymentMethod")}
          isSelectRequired={true}
          slot="firstFormElement"
          value={$form.paymentMethod}
          errors={$errors.paymentMethod}
        />
        <!-- TODO SELECT -->
        <FormInput
          name="amount"
          type="number"
          placeholder={$t("consultations.form.amount")}
          bind:value={$form.amount}
          errors={$errors.amount}
          slot="secondFormElement"
          {...$constraints.amount}
        />
      </FormLine>
      <div class="flex justify-end">
        <Button color="primary" type="button" on:click={handleUpdatePayment}
          >{$t(`payments.${action}.buttonAction`)}</Button
        >
      </div>
    </form>
  </TableBodyCell>
</TableBodyRow>
