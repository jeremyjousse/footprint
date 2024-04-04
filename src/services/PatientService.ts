import type { Patient, RowAction } from "$domain";
import {
  type TableColumn,
  type TableData,
  TableSort,
} from "$domain/valueObjects/DataTable";
import { invoke } from "@tauri-apps/api/core";

class PatientService {
  loadInitData = async (): Promise<TableData> => {
    const viewAction = () => {
      console.log("view action");
    };

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
      { name: "Add date", field: "createdAt" },
    ];

    const rowActions: RowAction[] = [{ name: "view", action: viewAction }];

    const filter = {
      lastName: "",
      firstName: "",
    };

    const rows: Patient[] = await this.list();

    return {
      columns,
      rows,
      rowActions,
      filter,
      sort,
      detailPath: "/patients/detail",
    };
  };

  list = async (): Promise<Patient[]> => {
    return await invoke<Patient[]>("patient_list_command", {});
  };

  get = async (id: string): Promise<Patient> => {
    return await invoke<Patient>("patient_detail_command", { id });
  };
}

const patientService = new PatientService();

export { patientService };
