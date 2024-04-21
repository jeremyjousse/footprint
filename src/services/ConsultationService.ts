import {
  CONSULTATION_LOCATION,
  CONSULTATION_STATUS,
  Toast,
  ToastTypes,
  type Consultation,
  type ConsultationAggregateDto,
} from "$domain";
import {
  type TableColumn,
  type TableData,
  TableSort,
} from "$domain/valueObjects/TableData";
import { addToast } from "$stores";
import { invoke } from "@tauri-apps/api/core";
import { t } from "$i18n";
import type { ConsultationAddCommandResult } from "$domain/dto/ConsultationAddCommandResult";
import { displayLocaleDateTime } from "$lib";
import { consultationStore } from "$stores/consultationStore";

class ConsultationService {
  loadInitData = async (patient: string): Promise<TableData> => {
    const sort: TableSort = new TableSort("createdAt");
    const columns: TableColumn[] = [
      {
        name: "Date",
        field: "appointmentDateTime",
        displayHelper: displayLocaleDateTime,
      },
      {
        name: "Type",
        field: "consultationType",
      },

      {
        name: "Price",
        field: "price",
      },
      {
        name: "Location",
        field: "location",
      },
      {
        name: "Status",
        field: "status",
      },
    ];

    const filter = {
      consultationType: "",
      appointmentDateTime: "",
      patient: patient,
    };

    const rows: Consultation[] = await this.list(patient);

    return {
      columns,
      rows,
      filter,
      sort,
    };
  };

  initNewConsultation = (patientId: string): ConsultationAggregateDto => {
    const consultation: Consultation = {
      appointmentDateTime: null,
      consultationType: "",
      location: "",
      note: null,
      patientId,
      price: null,
      status: CONSULTATION_STATUS.Pending,
    };

    const consultationAggregate: ConsultationAggregateDto = {
      consultation,
      payments: [],
    };
    consultationStore.set(consultationAggregate);

    return consultationAggregate;
  };

  add = async (consultation: Consultation): Promise<boolean> => {
    return await invoke<ConsultationAddCommandResult>(
      "consultation_add_command",
      {
        consultation: this.sanitizeConsultation(consultation),
      }
    )
      .then((message: ConsultationAddCommandResult) => {
        addToast(
          new Toast(
            // TODO add the patient name and the consultation date
            `${t.get("consultations.add.toastOk")}`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("consultations.add.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  update = async (consultation: Consultation): Promise<boolean> => {
    return await invoke<ConsultationAddCommandResult>(
      "consultation_update_command",
      {
        consultation: this.sanitizeConsultation(consultation),
      }
    )
      .then((message: ConsultationAddCommandResult) => {
        addToast(
          new Toast(
            // TODO add the patient name and the consultation date
            `${t.get("consultations.update.toastOk")}`,
            ToastTypes.Success
          )
        );
        return true;
      })
      .catch((error) => {
        addToast(
          new Toast(
            `${t.get("consultations.update.toastError")} ${error}`,
            ToastTypes.Error
          )
        );
        return false;
      });
  };

  list = async (patientId: String): Promise<Consultation[]> => {
    return await invoke<Consultation[]>("consultation_list_command", {
      filter: { patientId },
      sort: { by: "created_at", order: "Asc" },
    });
  };

  get = async (id: string): Promise<ConsultationAggregateDto> => {
    return await invoke<ConsultationAggregateDto>(
      "consultation_detail_command",
      { id }
    );
  };

  locations = () => {
    return Object.keys(CONSULTATION_LOCATION).map((value) => {
      return {
        [value]: $t(`consultations.form.locations.${value}`) as string,
      };
    });
  };

  private sanitizeConsultation(consultation: Consultation): Consultation {
    //   if (consultation.birthdate == "") {
    //     consultation.birthdate = null;
    //   }

    return consultation;
  }
}

const consultationService = new ConsultationService();

export { consultationService };
function $t(arg0: string): string {
  throw new Error("Function not implemented.");
}
