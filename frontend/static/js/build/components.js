"use strict";
class WebComponent extends HTMLElement {
    template;
    templateContent;
    constructor(templateId) {
        super();
        const maybeTemplate = document.getElementById(templateId);
        if (!maybeTemplate) {
            throw new Error(`template element with id=${templateId} not found`);
        }
        this.template = maybeTemplate;
        const templateContent = this.template.content.cloneNode(true);
        this.templateContent = templateContent;
    }
    connectedCallback() {
        this.appendChild(this.templateContent);
    }
}
function webComponentGenerator(templateId) {
    class Intermediate extends WebComponent {
        constructor() {
            super(templateId);
        }
    }
    customElements.define(templateId, Intermediate);
}
webComponentGenerator("close-cross-svg");
webComponentGenerator("info-svg");
webComponentGenerator("success-svg");
webComponentGenerator("warning-svg");
webComponentGenerator("error-svg");
