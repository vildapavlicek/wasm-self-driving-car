import { Config } from "wasm-self-driving-car";

export function initForm(document, config) {
  document.getElementById("lanesCountInput").value = config.lanesCount;
  document.getElementById("laneIndexInput").value = config.laneIndex;
  document.getElementById("carsCountInput").value = config.carsCount;
  document.getElementById("raysCountInput").value = config.raysCount;
  document.getElementById("raysLengthInput").value = config.raysLength;
  document.getElementById("raysSpread").value = config.raysSpread;
  document.getElementById("hiddenLayersInput").value = config.hiddenLayers;
  document.getElementById("mutationRateInput").value = config.mutationRate;
}

/* export function registerConfigUpdate(document) {
  let updateBtn = document.getElementById("updateBtn");

  updateBtn.addEventListener("click", () => {
    let config = new Config(
      document.getElementById("lanesCountInput").value,
      document.getElementById("laneIndexInput").value,
      document.getElementById("carsCountInput").value,
      document.getElementById("raysCountInput").value,
      document.getElementById("raysLengthInput").value,
      document
        .getElementById("neuronsCountInput")
        .value.split(",")
        .map((item) => parseInt(item, 10)),
      document.getElementById("mutationRateInput").value
    );
  });
} */

export function getConfigFromForm(document) {
  return new Config(
    parseInt(document.getElementById("lanesCountInput").value, 10),
    parseInt(document.getElementById("laneIndexInput").value, 10),
    parseInt(document.getElementById("carsCountInput").value, 10),
    parseInt(document.getElementById("raysCountInput").value, 10),
    parseInt(document.getElementById("raysLengthInput").value, 10),
    parseFloat(document.getElementById("raysSpread").value),
    document
      .getElementById("hiddenLayersInput")
      .value.split(",")
      .map((item) => parseInt(item, 10)),
    parseFloat(document.getElementById("mutationRateInput").value)
  );
}


export function freezeConfig(document) {
  document.getElementById("raysCountInput").disabled = true
  document.getElementById("raysLengthInput").disabled = true
  document.getElementById("raysLengthInput").disabled = true
  document.getElementById("raysSpread").disabled = true
  document.getElementById("hiddenLayersInput").disabled = true
}

export function unfreezeConfig(document) {
  document.getElementById("raysCountInput").disabled = false
  document.getElementById("raysLengthInput").disabled = false
  document.getElementById("raysLengthInput").disabled = false
  document.getElementById("raysSpread").disabled = false
  document.getElementById("hiddenLayersInput").disabled = false
}