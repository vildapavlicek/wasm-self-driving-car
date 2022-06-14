// import * as self_driving_car from "wasm-self-driving-car";
import {
  KeyEvent,
  Simulation,
  Config,
  SimulationState,
} from "wasm-self-driving-car";

import { getConfigFromForm, initForm } from "./formHandler";

let animationFrameId;

// initialize canvas
const carCanvas = document.getElementById("carCanvas");
carCanvas.width = 200;
carCanvas.height = window.innerHeight;
const carCtx = carCanvas.getContext("2d");

//
const networkCanvas = document.getElementById("networkCanvas");
const networkCtx = networkCanvas.getContext("2d");
networkCanvas.height = window.innerHeight;
networkCanvas.width = window.innerWidth * 0.4;
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

/* const verticalSpawnBtn = document.getElementById("verticalSpawnBtn");
verticalSpawnBtn.addEventListener("click", spawnVerticalCars); */

const horizontalSpawnBtn = document.getElementById("horizontalSpawnBtn");
horizontalSpawnBtn.addEventListener("click", spawnHorizontalCars);

const nextAgentBtn = document.getElementById("nextAgentBtn");
nextAgentBtn.addEventListener("click", nextAgent);

const previousAgentBtn = document.getElementById("previousAgentBtn");
previousAgentBtn.addEventListener("click", previousAgent);

const resetFocusBtn = document.getElementById("resetFocusBtn");
resetFocusBtn.addEventListener("click", resetFocus);

// TESTS SPAWNING
const easyTestBtn = document.getElementById("easyTestBtn");
easyTestBtn.addEventListener("click", easyTest);

const mediumTestBtn = document.getElementById("mediumTestBtn");
mediumTestBtn.addEventListener("click", mediumTest);

const hardTestBtn = document.getElementById("hardTestBtn");
hardTestBtn.addEventListener("click", hardTest);

const trainingTrafficBtn = document.getElementById("trainingTrafficBtn");
trainingTrafficBtn.addEventListener("click", trainingTraffic);

trainingTrafficBtn;

let simulation;
let config = Simulation.initConfig(window);

initForm(document, config);
generateTable(document);

const tbody = document.getElementById("rankingsTable");
tbody.addEventListener("click", function (e) {
  const cell = e.target.closest("td");
  if (!cell) {
    return;
  } // Quit, not clicked on a cell
  const row = cell.parentElement;

  if (simulation != null) {
    console.log("focusing agent", cell.innerHTML);
    simulation.focusAgent(parseInt(cell.innerHTML, 10));
  }
});

function animate() {
  carCanvas.height = window.innerHeight;
  networkCanvas.height = window.innerHeight;
  networkCanvas.width = window.innerWidth * 0.4;


  simulation.step(carCtx, networkCtx);

  const y = simulation.getFocusedAgentY();
  console.log("focused agent y", y);
  console.log("canvas height", carCanvas.height, "canvas width", carCanvas.width);
  carCtx.strokeStyle = "black";
  carCtx.beginPath();
  carCtx.moveTo(carCanvas.width / 2 - 10, carCanvas.height / 2 - 10);
  carCtx.lineTo(carCanvas.width / 2 + 10,  carCanvas.height / 2 - 10);
  carCtx.stroke();

  updateTable(document, simulation.top10Agents());

  animationFrameId = requestAnimationFrame(animate);
}

function save() {
  console.log("saving brain");
  simulation.saveFocusedCar(window);
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
}

function run() {
  if (simulation != null) {
    cancelAnimationFrame(animationFrameId);
  }

  simulation = new Simulation(
    carCanvas.width,
    window,
    getConfigFromForm(document)
  );
  //simulation.addTestTraffic();
  simulation.run();
  animate();
  return;
}

/* function spawnVerticalCars() {
  let lane_ids = document
    .getElementById("verticalSpawnerLaneIdInput")
    .value.split(",")
    .map((item) => parseInt(item, 10));
  console.log("lane id parsed", lane_ids);
  simulation.spawnCarsVertically(lane_ids);
}
 */
function spawnHorizontalCars() {
  let lane_ids = document
    .getElementById("horizontalSpawnerLaneIdInput")
    .value.split(",")
    .map((item) => parseInt(item, 10));
  // let lane_id = parseInt(document.getElementById("spawnLaneIdInput").value, 10);
  console.log("lane id parsed", lane_ids);
  simulation.spawnCarsHorizontally(lane_ids);
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
    table.rows[i].cells[0].innerHTML = i + 1 + ".";
    table.rows[i].cells[1].innerHTML = rankings[i];
  }
}

function resetFocus() {
  simulation.resetFocus();
}

const EASY = 1;
const MEDIUM = 0.75;
const HARD = 0.5;

function easyTest() {
  if (simulation == null) {
    return;
  }
  simulation.addTestTraffic(EASY);
}

function mediumTest() {
  if (simulation == null) {
    return;
  }
  simulation.addTestTraffic(MEDIUM);
}

function hardTest() {
  if (simulation == null) {
    return;
  }
  simulation.addTestTraffic(HARD);
}

function trainingTraffic() {
  if (simulation == null) {
    return;
  }
  simulation.trainingTraffic();
}
