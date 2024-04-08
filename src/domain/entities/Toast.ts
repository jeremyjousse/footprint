import { ToastTypes } from "$domain";

export class Toast {
  id: number;
  dismissible: boolean;
  message: string;
  type: ToastTypes;
  timeout: number;

  constructor(
    message: string,
    type: ToastTypes,
    dismissible = true,
    timeout = 3000
  ) {
    const id = Math.floor(Math.random() * 10000);

    this.id = id;
    this.dismissible = true;
    this.message = message;
    this.type = type;
    this.dismissible = dismissible;
    this.timeout = timeout;
  }
}
