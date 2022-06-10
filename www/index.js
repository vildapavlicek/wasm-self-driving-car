// import * as self_driving_car from "wasm-self-driving-car";
import {
  KeyEvent,
  Simulation,
  Config,
  SimulationState,
} from "wasm-self-driving-car";

import { getConfigFromForm, initForm } from "./formHandler";

const LANES_COUNT_DEFAULT = 3;
const LANE_INDEX_DEFAULT = 1;
const CARS_COUNT_DEFAULT = 100;
const RAYS_COUNT = 5;
const RAY_LENGTH = 120;
const RAYS_SPREAD_DEFAULT = 2;
const HIDDEN_LAYERS_DEFAULT = [6];
const MUTATION_RATE_DEFAULT = 0.2;
// DO NOT CHANGE, we have 4 directions we can go in
// and we mapped 1 output neuron to each direction
const OUTPUT_LAYER_NEURONS = 4;

let animationFrameId;

// initialize canvas
const carCanvas = document.getElementById("carCanvas");
carCanvas.width = 200;
const carCtx = carCanvas.getContext("2d");
//
const networkCanvas = document.getElementById("networkCanvas");
const networkCtx = networkCanvas.getContext("2d");

//
const startPauseBtn = document.getElementById("startPause");
startPauseBtn.addEventListener("click", startPause);

const stopBtn = document.getElementById("stop");
stopBtn.addEventListener("click", stop);

const save_btn = document.getElementById("save");
save_btn.addEventListener("click", save);

const discard_btn = document.getElementById("discard");
discard_btn.addEventListener("click", discard);

const runBtn = document.getElementById("runBtn");
runBtn.addEventListener("click", run);

const spawnBtn = document.getElementById("spawnBtn");
spawnBtn.addEventListener("click", spawn);

const nextAgentBtn = document.getElementById("nextAgentBtn");
nextAgentBtn.addEventListener("click", nextAgent);

const previousAgentBtn = document.getElementById("previousAgentBtn");
previousAgentBtn.addEventListener("click", previousAgent);

let simulation;
let config = new Config(
  LANES_COUNT_DEFAULT,
  LANE_INDEX_DEFAULT,
  CARS_COUNT_DEFAULT,
  RAYS_COUNT,
  RAY_LENGTH,
  RAYS_SPREAD_DEFAULT,
  HIDDEN_LAYERS_DEFAULT,
  MUTATION_RATE_DEFAULT
);

initForm(document, config);

generateTable(document);

const tbody = document.getElementById("rankingsTable");
tbody.addEventListener('click', function (e) {
  const cell = e.target.closest('td');
  if (!cell) {return;} // Quit, not clicked on a cell
  const row = cell.parentElement;

  if (simulation != null) {
    console.log("focusing agent", cell.innerHTML);
    simulation.focusAgent(parseInt(cell.innerHTML, 10))
  }

});

function animate() {
  carCanvas.height = window.innerHeight;
  networkCanvas.height = window.innerHeight;
  networkCanvas.width = window.innerWidth * 0.4;
  simulation.step(carCtx, networkCtx);
  updateTable(document, simulation.top10Agents());
  animationFrameId = requestAnimationFrame(animate);
}

function resize() {
  carCanvas.height = window.innerHeight;
  networkCanvas.height = window.innerHeight;
  networkCanvas.width = window.innerWidth * 0.4;
}

function save() {
  console.log("saving brain");
  simulation.save_best_focused_car(window);
  alert("brain saved");
}

function discard() {
  console.log("discarding brain");
  simulation.discard_brain(window);
}

function startPause() {
  if (simulation == null) {
    console.log("simulation is null, doing nothing");
    return;
  }

  switch (simulation.state) {
    case SimulationState.Stopped:
      break;

    case SimulationState.Running:
      simulation.pause();
      break;

    case SimulationState.Paused:
      simulation.run();
      break;
  }
}

function stop() {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
  }

  simulation.stop();
  simulation = null;
  console.log("simulation destroyed", simulation == null);
}

function run() {
  if (simulation != null) {
    cancelAnimationFrame(animationFrameId);
  }

  simulation = new Simulation(
    carCanvas.width,
    window,
    getConfigFromForm(document)
  ).add_basic_traffic();
  simulation.run();
  animate();
  return;
}

function spawn() {
  simulation.spawn_car(0);
}

function nextAgent() {
  simulation.next_agent();
}

function previousAgent() {
  simulation.previous_agent();
}

function generateTable(document) {
  let rankingsDiv = document.getElementById("rankings");
  let table = document.createElement("table");
  // table.classList.add('rankingsTable');
  table.setAttribute("id", "rankingsTable");

  for (let i = 0; i < 10; i++) {
    const tr = table.insertRow();
    tr.insertCell().innerHTML = i + 1;
    tr.insertCell().innerHTML = null;
    rankingsDiv.appendChild(table);
  }
}

function updateTable(document, rankings) {
  let table = document.getElementById("rankingsTable");

  for (let i = 0; i < rankings.length; i++) {
    table.rows[i].cells[0].innerHTML = i + 1 + '.';
    table.rows[i].cells[1].innerHTML = rankings[i];
  }
}
