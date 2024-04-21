<script lang="ts">
  import { t } from "$i18n";
  import Breadcrumb from "$components/molecules/Breadcrumbs.svelte";
  import BankAccountForm from "$components/organisms/bankAccounts/BankAccountForm.svelte";
  import type { BankAccountAddLoadData } from "./+page";
  import { bankAccountService } from "$services";
  import { DETAIL_ACTIONS } from "$domain";

  export let data: BankAccountAddLoadData;
  let bankAccount = bankAccountService.initNewBankAccount();
</script>

<Breadcrumb breadcrumbs={data.breadcrumbs} />
<BankAccountForm {bankAccount} action={DETAIL_ACTIONS.Add} />
