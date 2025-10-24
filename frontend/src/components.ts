class WebComponent extends HTMLElement {
  private template: HTMLTemplateElement;
  private templateContent: DocumentFragment;

  constructor(templateId: string) {
    super();
    const maybeTemplate = document.getElementById(templateId) as HTMLTemplateElement | null;

    if (!maybeTemplate) {
      throw new Error(`template element with id=${templateId} not found`);
    }

    this.template = maybeTemplate;

    const templateContent = this.template.content.cloneNode(true) as DocumentFragment;

    this.templateContent = templateContent;
  }

  connectedCallback() {
    this.appendChild(this.templateContent);
  }
}

function webComponentGenerator(templateId: string) {
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
