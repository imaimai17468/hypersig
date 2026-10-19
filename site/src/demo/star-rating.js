// A star rating written as a plain custom element, for the demo of a script
// on a hypersig page. It shows the `value` attribute as filled stars and
// dispatches `rate` with the number of the star clicked as its `detail`.
class StarRating extends HTMLElement {
  static observedAttributes = ["value"];

  constructor() {
    super();
    const root = this.attachShadow({ mode: "open" });
    root.innerHTML = `<style>
      button { font: inherit; font-size: 1.75rem; line-height: 1; padding: 0 0.1rem;
        border: 0; background: none; cursor: pointer; color: #d4d4d8; }
      button.on { color: #f59e0b; }
    </style>`;
    for (let star = 1; star <= 5; star++) {
      const button = document.createElement("button");
      button.textContent = "★";
      button.setAttribute("aria-label", `${star} of 5`);
      button.addEventListener("click", () => {
        this.dispatchEvent(new CustomEvent("rate", { detail: star }));
      });
      root.append(button);
    }
  }

  attributeChangedCallback() {
    const value = Number(this.getAttribute("value") ?? 0);
    this.shadowRoot.querySelectorAll("button").forEach((button, i) => {
      button.classList.toggle("on", i < value);
    });
  }
}

customElements.define("star-rating", StarRating);
