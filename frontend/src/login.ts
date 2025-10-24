const checkbox = document.querySelector("#login-form");

checkbox?.addEventListener("submit", login, false);

async function login(event: Event) {
  event.preventDefault();
  const currentTarget = (
    event as SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ).currentTarget;

  const formData = new FormData(currentTarget);
  const formEntries = Object.fromEntries(formData.entries());
  console.log(formEntries);

  const res = await fetch("/api/login", {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(formEntries),
  });
  console.log(res);
  const json = await res.json();
  console.log(json);
}
