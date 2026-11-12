// Defines <mic-level>, which listens to the microphone and draws how loud
// it is. It sets `data-state` to "listening" once the microphone is open, or
// to the name of the error getUserMedia threw, and dispatches the same text
// as the `state` event.
class MicLevel extends HTMLElement {
  async connectedCallback() {
    const bar = document.createElement("div");
    bar.className = "bar";
    this.append(bar);
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const context = new AudioContext();
      const analyser = context.createAnalyser();
      context.createMediaStreamSource(stream).connect(analyser);
      const samples = new Float32Array(analyser.fftSize);
      const draw = () => {
        analyser.getFloatTimeDomainData(samples);
        const peak = samples.reduce((loudest, sample) => Math.max(loudest, Math.abs(sample)), 0);
        bar.style.inlineSize = `${Math.round(peak * 100)}%`;
        requestAnimationFrame(draw);
      };
      draw();
      this.report("listening");
    } catch (error) {
      this.report(error.name);
    }
  }

  report(state) {
    this.dataset.state = state;
    this.dispatchEvent(new CustomEvent("state", { detail: state }));
  }
}

customElements.define("mic-level", MicLevel);

// Defines <screen-share>, a button that shares the screen and shows what
// it shares under it. It sets `data-state` to "sharing" once the screen is
// shared, or to the name of the error getDisplayMedia threw.
class ScreenShare extends HTMLElement {
  connectedCallback() {
    const button = document.createElement("button");
    button.textContent = "Share the screen";
    const video = document.createElement("video");
    video.muted = true;
    video.autoplay = true;
    button.addEventListener("click", () => this.share(video));
    this.append(button, video);
  }

  async share(video) {
    try {
      video.srcObject = await navigator.mediaDevices.getDisplayMedia({ video: true });
      this.dataset.state = "sharing";
    } catch (error) {
      this.dataset.state = error.name;
    }
  }
}

customElements.define("screen-share", ScreenShare);
