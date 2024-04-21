import {
  DataTableColumnFilterType,
  PAYMENT_METHOD,
  PAYMENT_STATUS,
  TableSort,
  Toast,
  ToastTypes,
  type Payment,
  type TableColumn,
  type TableData,
} from "$domain";
import type { PaymentAddCommandResult } from "$domain/dto/PaymentAddCommandResult";
import { t } from "$i18n";
import { addToast } from "$stores";
import { invoke } from "@tauri-apps/api/core";

class PaymentService {
  loadInitData = async (): Promise<TableData> => {
    const sort: TableSort = new TableSort("createdAt");

    const columns: TableColumn[] = [
      {
        name: "Payment method",
        field: "paymentMethod",
        filter: { type: DataTableColumnFilterType.String },
      },
    ];

    const filter = {
      name: "",
    };

    const rows: Payment[] = await this.list();

    return {
      columns,
      rows,
      filter,
      sort,
      detailPath: "/payments/detail",
    };
  };

  initNewPayment = (): Payment => {
    const payment: Payment = {
      id: undefined,
      consultationId: "",
      payedAt: "",
      paymentMethod: PAYMENT_METHOD.Cash,
      amount: 0.0,
      status: PAYMENT_STATUS.Pending,
    };

    return payment;
  };

  list = async (): Promise<Payment[]> => {
    return await invoke<Payment[]>("payment_list_command", {});
  };

  get = async (id: string): Promise<Payment> => {
    return await invoke<Payment>("payment_detail_command", {
      id,
    });
  };

  add = async (payment: Payment): Promise<boolean> => {
    return await invoke<Payment>("payment_add_command", {
      payment,
    })
      .then((message: PaymentAddCommandResult) => {
        addToast(
          new Toast(`${t.get("payments.add.toastOk")}`, ToastTypes.Success)
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("payments.add.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  update = async (payment: Payment): Promise<boolean> => {
    return await invoke<Payment>("payment_update_command", {
      payment,
    })
      .then((message: PaymentAddCommandResult) => {
        addToast(
          new Toast(`${t.get("payments.update.toastOk")}`, ToastTypes.Success)
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("payments.update.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };
}

const paymentService = new PaymentService();

export { paymentService };
