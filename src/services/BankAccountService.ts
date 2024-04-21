import { t } from "$i18n";
import {
  DataTableColumnFilterType,
  TableSort,
  Toast,
  ToastTypes,
  type BankAccountAddCommandResult,
  type TableColumn,
  type TableData,
} from "$domain";

import type { BankAccount } from "$domain/entities/BankAccount";
import { addToast } from "$stores";
import { invoke } from "@tauri-apps/api/core";

class BankAccountService {
  loadInitData = async (): Promise<TableData> => {
    const sort: TableSort = new TableSort("createdAt");
    const columns: TableColumn[] = [
      {
        name: "Bank name",
        field: "bankName",
        filter: { type: DataTableColumnFilterType.String },
      },
      {
        name: "Account number",
        field: "accountNumber",
        filter: { type: DataTableColumnFilterType.String },
      },
    ];

    const filter = {
      name: "",
    };

    const rows: BankAccount[] = await this.list();

    return {
      columns,
      rows,
      filter,
      sort,
      detailPath: "/bank-accounts/detail",
    };
  };

  initNewBankAccount = (): BankAccount => {
    return {
      id: undefined,
      bankName: "",
      accountNumber: "",
      bankChequeDepositNumber: 0,
    } as BankAccount;
  };

  list = async (): Promise<BankAccount[]> => {
    console.log("ssss");
    return await invoke<BankAccount[]>("bank_account_list_command", {});
  };

  get = async (id: string): Promise<BankAccount> => {
    return await invoke<BankAccount>("bank_account_detail_command", {
      id,
    });
  };

  add = async (bankAccount: BankAccount): Promise<boolean> => {
    return await invoke<BankAccount>("bank_account_add_command", {
      bankAccount,
    })
      .then((message: BankAccountAddCommandResult) => {
        addToast(
          new Toast(
            `${t.get("BankAccounts.add.toastOk", { name: message.name })}`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("BankAccounts.add.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  update = async (bankAccount: BankAccount): Promise<BankAccount> => {
    return await invoke<BankAccount>("bank_account_update_command", {
      bankAccount,
    });
  };
}

const bankAccountService = new BankAccountService();

export { bankAccountService };
