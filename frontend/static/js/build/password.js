"use strict";
document.querySelectorAll("input[data-id=password]").forEach((element) => {
    element.addEventListener("keydown", (event) => {
        if (event.ctrlKey) {
            window.dispatchEvent(new CustomEvent("showpassword", {
                detail: {
                    value: "true",
                },
            }));
        }
    });
    element.addEventListener("keyup", () => {
        window.dispatchEvent(new CustomEvent("showpassword", {
            detail: {
                value: "false",
            },
        }));
    });
});
