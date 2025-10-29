import { WebComponentRegistery } from "$lib/components.js";
import { Toaster } from "$lib/toaster.js";
import { defaultSetup } from "$utils/defaultSetup.js";

class CreateSellerForm extends HTMLElement {
  async handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const target = (
      event as SubmitEvent & { target: EventTarget & HTMLFormElement }
    ).target;

    if (!target) {
      return;
    }

    const formData = new FormData(target);
    const { "seller-name": sellerName } = Object.fromEntries(
      formData.entries(),
    );
    // console.log(Object.fromEntries(formData.entries()));

    const res = await fetch("/api/seller", {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify([
        {
          name: sellerName?.toLowerCase(),
        },
      ]),
    });
    // console.log(res);
    const json = await res.json();
    // console.log(json);

    const toaster = Toaster.getInstance();
    if (json?.result?.success === true) {
      toaster.success("Successfully created seller");
    } else {
      toaster.error("Error occured while creating seller");
    }

    // if (json?.error?.message === "FAIL") {
    //   console.log("Error occured while creating seller");
    // }
  }

  connectedCallback() {
    this.onsubmit = this.handleSubmit;
  }
}

function setup() {
  defaultSetup();
  WebComponentRegistery.register("create-seller-form", CreateSellerForm);
}

setup();
