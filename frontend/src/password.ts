document.querySelectorAll("input[data-id=password]").forEach((element) => {
  (element as HTMLInputElement).addEventListener("keydown", (event) => {
    if (event.ctrlKey) {
      window.dispatchEvent(
        new CustomEvent("showpassword", {
          detail: {
            value: "true",
          },
        }),
      );
    }
  });

  (element as HTMLInputElement).addEventListener("keyup", () => {
    window.dispatchEvent(
      new CustomEvent("showpassword", {
        detail: {
          value: "false",
        },
      }),
    );
  });
});
