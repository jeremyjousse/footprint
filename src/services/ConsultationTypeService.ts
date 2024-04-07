import { t } from "$i18n";
import {
  TableSort,
  Toast,
  ToastTypes,
  type ConsultationTypeAddCommandResult,
  type TableColumn,
  type TableData,
} from "$domain";

import type { ConsultationType } from "$domain/entities/ConsultationType";
import { addToast } from "$stores";
import { invoke } from "@tauri-apps/api/core";

class ConsultationTypeService {
  loadInitData = async (): Promise<TableData> => {
    const sort: TableSort = new TableSort("createdAt");
    const columns: TableColumn[] = [
      {
        name: "Last name",
        field: "lastName",
        filter: { type: "string" },
      },
      {
        name: "First name",
        field: "firstName",
        filter: { type: "string" },
      },
    ];

    const filter = {
      name: "",
    };

    const rows: ConsultationType[] = await this.list();

    return {
      columns,
      rows,
      filter,
      sort,
      detailPath: "/consultation_types/detail",
    };
  };

  initNewConsultationType = (): ConsultationType => {
    const consultationType: ConsultationType = {
      name: "",
      price: 0.0,
    };

    return consultationType;
  };

  list = async (): Promise<ConsultationType[]> => {
    return await invoke<ConsultationType[]>(
      "consultation_type_list_command",
      {}
    );
  };

  get = async (id: string): Promise<ConsultationType> => {
    return await invoke<ConsultationType>("consultation_type_detail_command", {
      id,
    });
  };

  add = async (
    consultationType: ConsultationType
  ): Promise<ConsultationType> => {
    return await invoke<ConsultationType>("consultation_type_add_command", {
      consultationType,
    })
      .then((message: ConsultationTypeAddCommandResult) => {
        addToast(
          new Toast(
            `${t.get("consultationTypes.add.toastOk", { name: message.name })}`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("consultationTypes.add.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  update = async (
    consultationType: ConsultationType
  ): Promise<ConsultationType> => {
    return await invoke<ConsultationType>("consultation_type_update_command", {
      consultationType,
    });
  };
}

const consultationTypeService = new ConsultationTypeService();

export { consultationTypeService };
