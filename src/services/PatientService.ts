import type { Patient, RowAction } from "$domain";
import {
  type TableColumn,
  type TableData,
  TableSort,
} from "$domain/valueObjects/DataTable";
import { invoke } from "@tauri-apps/api/core";

class PatientService {
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
      lastName: "",
      firstName: "",
    };

    const rows: Patient[] = await this.list();

    return {
      columns,
      rows,
      filter,
      sort,
      detailPath: "/patients/detail",
    };
  };

  initNewPatient = (): Patient => {
    const patient: Patient = {
      birthdate: null,
      createdAt: null,
      contactInformation: {
        email: null,
        phone: null,
        mobilePhone: null,
      },
      diabetic: false,
      longDurationDisease: false,
      nationalInsuranceNumber: null,
      notes: "",
      personalName: {
        firstName: "",
        lastName: "",
      },
      postalAddress: {
        city: null,
        country: null,
        postalCode: null,
        street: null,
      },
      updatedAt: null,
    };
    return patient;
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
