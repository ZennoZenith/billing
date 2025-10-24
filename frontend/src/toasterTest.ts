import { TOAST_TYPES, Toaster } from "$lib/toaster.js";
import type { ToastType } from "$type/index.js";
import { exhaustiveMatchingGuard } from "$utils/helpers.js";

export class ToastTest extends HTMLElement {
  private toastTestTemplate: HTMLTemplateElement;

  constructor() {
    super();
    const maybeTemplate = document.getElementById(
      "test-toast-template",
    ) as HTMLTemplateElement | null;

    if (!maybeTemplate) {
      throw new Error("toast-container-template not found");
    }

    this.toastTestTemplate = maybeTemplate;
  }
  connectedCallback() {
    const templateContent = this.toastTestTemplate.content.cloneNode(
      true,
    ) as HTMLElement;
    this.appendChild(templateContent);
    const elements = document.querySelectorAll("[data-emmit-toast-type]");
    for (const element of elements) {
      const toastType = element.getAttribute("data-emmit-toast-type");
      if (toastType === null) continue;
      if (!TOAST_TYPES.includes(toastType as ToastType)) continue;

      element.addEventListener("click", () =>
        showToast(toastType as ToastType),
      );
    }
  }
}
customElements.define("toast-test", ToastTest);

function showToast(toastType: ToastType) {
  const toaster = Toaster.getInstance();
  switch (toastType) {
    case "INFO":
      toaster.info("Info Message", "Info title", 0);
      break;
    case "SUCCESS":
      toaster.success("Success Message", "Success title");
      break;
    case "WARNING":
      toaster.warning("Warning Message", "Warning title");
      break;
    case "ERROR":
      toaster.error("Error Message", "Error title");
      break;
    default:
      exhaustiveMatchingGuard(toastType);
  }
}
