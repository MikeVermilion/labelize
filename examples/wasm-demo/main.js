import init, { render_zpl_to_png } from "../../pkg/labelize.js";

const elements = {
  zpl: document.querySelector("#zpl-input"),
  widthMm: document.querySelector("#width-mm"),
  heightMm: document.querySelector("#height-mm"),
  dpmm: document.querySelector("#dpmm"),
  invert: document.querySelector("#invert"),
  renderButton: document.querySelector("#render-button"),
  previewImage: document.querySelector("#preview-image"),
  status: document.querySelector("#status"),
  downloadLink: document.querySelector("#download-link"),
};

let activeUrl = null;

function setStatus(message, isError = false) {
  elements.status.textContent = message;
  elements.status.classList.toggle("error", isError);
}

function revokeActiveUrl() {
  if (activeUrl) {
    URL.revokeObjectURL(activeUrl);
    activeUrl = null;
  }
}

function numberValue(input) {
  const value = input.valueAsNumber;
  return Number.isFinite(value) ? value : undefined;
}

async function render() {
  elements.renderButton.disabled = true;
  setStatus("Rendering...");

  try {
    const pngBytes = render_zpl_to_png(
      elements.zpl.value,
      numberValue(elements.widthMm),
      numberValue(elements.heightMm),
      numberValue(elements.dpmm),
      elements.invert.checked
    );

    revokeActiveUrl();
    activeUrl = URL.createObjectURL(new Blob([pngBytes], { type: "image/png" }));
    elements.previewImage.src = activeUrl;
    elements.downloadLink.href = activeUrl;
    elements.downloadLink.hidden = false;
    setStatus(`Rendered ${pngBytes.length.toLocaleString()} bytes.`);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    setStatus(message, true);
  } finally {
    elements.renderButton.disabled = false;
  }
}

await init();
elements.renderButton.addEventListener("click", render);
window.addEventListener("beforeunload", revokeActiveUrl);

render();
