// import * as self_driving_car from "wasm-self-driving-car";
import {
  KeyEvent,
  Simulation,
  Config,
  SimulationState,
} from "wasm-self-driving-car";

const RAYS_COUNT = 5;
const RAY_LENGTH = 120;
// DO NOT CHANGE, we have 4 directions we can go in
// and we mapped 1 output neuron to each direction
const OUTPUT_LAYER_NEURONS = 4;

// initialize canvas
const carCanvas = document.getElementById("carCanvas");
carCanvas.width = 200;
const carCtx = carCanvas.getContext("2d");

const networkCanvas = document.getElementById("networkCanvas");
networkCanvas.width = 600;
const networkCtx = networkCanvas.getContext("2d");

const startPauseBtn = document.getElementById("startPause");
startPauseBtn.addEventListener("click", startPause);

const stopBtn = document.getElementById("stop");
stopBtn.addEventListener("click", stop);

const save_btn = document.getElementById("save");
save_btn.addEventListener("click", save);

const discard_btn = document.getElementById("discard");
discard_btn.addEventListener("click", discard);

let config = new Config(
  3,
  100,
  RAYS_COUNT,
  RAY_LENGTH,
  [RAYS_COUNT, 6, OUTPUT_LAYER_NEURONS],
  0.2
);

let simulation = new Simulation(
  carCanvas.width,
  window,
  config
).add_basic_traffic();
simulation.run();

addKeyboardListeners();
simulationStep();
// animate();

function simulationStep() {
  carCanvas.height = window.innerHeight;
  networkCanvas.height = window.innerHeight;

  simulation.step(carCtx, networkCtx);
  requestAnimationFrame(simulationStep);
}

function addKeyboardListeners() {
  console.log("addKeyboardListeners");
  document.onkeydown = (e) => {
    let key_event = null;
    switch (e.key) {
      case "ArrowUp":
        key_event = KeyEvent.UpPressed;
        break;
      case "ArrowLeft":
        key_event = KeyEvent.LeftPressed;
        break;
      case "ArrowRight":
        key_event = KeyEvent.RightPressed;
        break;
      case "ArrowDown":
        key_event = KeyEvent.DownPressed;
        break;
    }

    if (key_event != null) {
      car.handle_key_input(key_event);
    }
  };

  document.onkeyup = (e) => {
    let key_event = null;
    switch (e.key) {
      case "ArrowUp":
        key_event = KeyEvent.UpReleased;
        break;
      case "ArrowLeft":
        key_event = KeyEvent.LeftReleased;
        break;
      case "ArrowRight":
        key_event = KeyEvent.RightReleased;
        break;
      case "ArrowDown":
        key_event = KeyEvent.DownReleased;
        break;
    }

    if (key_event != null) {
      car.handle_key_input(key_event);
    }
  };
}

function save() {
  console.log("saving brain");
  simulation.save_best_car(window);
  alert("brain saved");
}

function discard() {
  console.log("discarding brain");
  simulation.discard_brain(window);
}

function startPause() {
  console.log(simulation.state);
  switch (simulation.state) {
    case SimulationState.Stopped:
        console.log("window", window, "config", config, "simulation:", simulation);
      simulation = new Simulation(
        carCanvas.width,
        window,
        new Config(
            3,
            100,
            RAYS_COUNT,
            RAY_LENGTH,
            [RAYS_COUNT, 6, OUTPUT_LAYER_NEURONS],
            0.2
          )
      ).add_basic_traffic();
      simulation.run();
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
  simulation.stop();
}
