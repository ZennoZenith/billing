import { Toaster } from "$lib/toaster.js";

const registerForm = document.querySelector("#register-form");

registerForm?.addEventListener("submit", register, false);

async function register(event: Event) {
  event.preventDefault();
  const currentTarget = (
    event as SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ).currentTarget;

  const formData = new FormData(currentTarget);
  const formEntries = Object.fromEntries(formData.entries());
  console.log(formEntries);

  const res = await fetch("/api/register", {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(formEntries),
  });
  const json = await res.json();

  if (json?.result?.success) {
    Toaster.getInstance().success("Registered successfully");
    currentTarget.reset();
  }

  if (json?.error?.message === "USER_ALREADY_EXISTS") {
    window.dispatchEvent(
      new CustomEvent("registerpost", {
        detail: {
          error: { email: "true" },
        },
      }),
    );
  }
}
