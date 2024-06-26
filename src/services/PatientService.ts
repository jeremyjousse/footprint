import {
  Toast,
  ToastTypes,
  type Patient,
  type PatientAddCommandResult,
  type RowAction,
} from "$domain";
import {
  type TableColumn,
  type TableData,
  TableSort,
} from "$domain/valueObjects/TableData";
import { addToast } from "$stores";
import { invoke } from "@tauri-apps/api/core";
import { t } from "$i18n";

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

  add = async (patient: Patient): Promise<boolean> => {
    return await invoke<PatientAddCommandResult>("patient_add_command", {
      patient: this.sanitizePatient(patient),
    })
      .then((message: PatientAddCommandResult) => {
        addToast(
          new Toast(
            `${t.get("patient.add.toastOk", { name: `${message.personalName.lastName} ${message.personalName.firstName}` })} added`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("patient.add.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  // TODO merge update and add with a private function
  // extends from a tauri command service
  update = async (patient: Patient): Promise<boolean> => {
    return await invoke<PatientAddCommandResult>("patient_update_command", {
      patient: this.sanitizePatient(patient),
    })
      .then((message: PatientAddCommandResult) => {
        addToast(
          new Toast(
            `${t.get("patient.update.toastOk", { name: `${message.personalName.lastName} ${message.personalName.firstName}` })} added`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("patient.update.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  list = async (): Promise<Patient[]> => {
    return await invoke<Patient[]>("patient_list_command", {});
  };

  get = async (id: string): Promise<Patient> => {
    return await invoke<Patient>("patient_detail_command", { id });
  };

  private sanitizePatient(patient: Patient): Patient {
    if (patient.birthdate == "") {
      patient.birthdate = null;
    }

    return patient;
  }
}

const patientService = new PatientService();

export { patientService };
