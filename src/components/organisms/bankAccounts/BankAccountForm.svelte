<script lang="ts">
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Label from "$components/atoms/form/Label.svelte";
  import FormLine from "$components/molecules/FormLine.svelte";
  import {
    BankAccountSchema,
    type BankAccount,
    type DetailActions,
    DETAIL_ACTIONS,
  } from "$domain";
  import { t } from "$i18n";
  import { superForm } from "sveltekit-superforms";
  import { valibot } from "sveltekit-superforms/adapters";
  import { bankAccountService } from "$services";
  import Button from "$components/atoms/Button.svelte";

  export let bankAccount: BankAccount;
  export let reloadListAction: () => void;

  export let action: DetailActions = DETAIL_ACTIONS.Add;

  const { form, enhance, errors } = superForm(bankAccount, {
    validators: valibot(BankAccountSchema),
    dataType: "json",
  });

  const handleUpdateBankAccount = async () => {
    let callSuccess = false;

    if (action == DETAIL_ACTIONS.Add) {
      callSuccess = await bankAccountService.add($form);
    } else if (action == DETAIL_ACTIONS.Edit) {
      callSuccess = await bankAccountService.update($form);
    }
  };
</script>

<form
  method="POST"
  use:enhance
  on:submit|preventDefault={handleUpdateBankAccount}
>
  <FormLine>
    <Label slot="label">{$t("bankAccounts.form.bankName")}</Label>
    <FormInput
      name="bankName"
      type="text"
      min="1"
      step="any"
      placeholder={$t("bankAccounts.form.bankName")}
      bind:value={$form.bankName}
      errors={$errors.bankName}
      slot="firstFormElement"
    />
    <FormInput
      name="accountNumber"
      type="text"
      min="1"
      step="any"
      placeholder={$t("bankAccounts.form.accountNumber")}
      bind:value={$form.accountNumber}
      errors={$errors.accountNumber}
      slot="secondFormElement"
    />
  </FormLine>
  <FormLine>
    <Label slot="label">{$t("bankAccounts.form.bankChequeDepositNumber")}</Label
    >
    <FormInput
      name="bankChequeDepositNumber"
      type="number"
      min="1"
      step="any"
      placeholder={$t("bankAccounts.form.bankChequeDepositNumber")}
      bind:value={$form.bankChequeDepositNumber}
      errors={$errors.bankChequeDepositNumber}
      slot="firstFormElement"
    />
  </FormLine>
  <div class="flex justify-end">
    <Button color="primary" type="button" on:click={handleUpdateBankAccount}
      >{$t(`bankAccounts.${action.toLocaleLowerCase()}.buttonAction`)}</Button
    >
  </div>
</form>
