 // import * as self_driving_car from "wasm-self-driving-car";
 import { Car, KeyEvent, Road, Border, Traffic, Level, Visualizer, NeuralNetwork } from "wasm-self-driving-car";

 // initialize canvas
 const carCanvas = document.getElementById("carCanvas");
 carCanvas.width = 200;
 const carCtx = carCanvas.getContext("2d");


 const networkCanvas = document.getElementById("networkCanvas");
 networkCanvas.width = 300;
 const networkCtx = networkCanvas.getContext("2d");
 

// init road
const road = Road.new(carCanvas.width / 2, carCanvas.width * 0.9, 3);
// get our car
//const car = Car.ai_controlled(road.lane_center(1), 100, 30, 50);
const cars = gen_cars(1000);

let bestCar = cars[0];


const save_btn = document.getElementById("save");
save_btn.addEventListener("click", save);

const discard_btn = document.getElementById("discard");
discard_btn.addEventListener("click", discard);

if (localStorage.getItem("bestBrain")) {

    const brain = NeuralNetwork.deserialize_brain(localStorage.getItem("bestBrain"));
    cars.forEach(c => c.set_brain(brain.mutate(0.3)));
    cars[0].set_brain(brain);

    //bestCar.deserialize_brain(localStorage.getItem("bestBrain"));
}

// other cars
const traffic = Traffic.new();

traffic.add(Car.no_control(road.lane_center(0), -100, 30, 50, 2));
traffic.add(Car.no_control(road.lane_center(2), -100, 30, 50, 2));

traffic.add(Car.no_control(road.lane_center(1), -250, 30, 50, 2));

traffic.add(Car.no_control(road.lane_center(0), -500, 30, 50, 2));
traffic.add(Car.no_control(road.lane_center(1), -500, 30, 50, 2));

traffic.add(Car.no_control(road.lane_center(2), -650, 30, 50, 2));

addKeyboardListeners();
animate();

function animate(time) {

    traffic.update(road);

    cars.forEach(car => { car.update(road, traffic); });

    bestCar = cars.find(
        c => c.y == Math.min(
            ...cars.map(c=>c.y)
        )
    );

    carCanvas.height = window.innerHeight;
    networkCanvas.height = window.innerHeight;

    carCtx.save();
    carCtx.translate(0, -bestCar.y + carCanvas.height * 0.7);
    road.draw(carCtx);

    traffic.draw(carCtx);

    carCtx.globalAlpha = 0.2;
    cars.forEach(car => { car.draw(carCtx, false); });
    carCtx.globalAlpha = 1;

    bestCar.draw(carCtx, true);

    carCtx.restore();

    networkCtx.lineDashOffset = time / -50;
    Visualizer.draw_network(networkCtx, bestCar.brain());
    requestAnimationFrame(animate);
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
           }
              
           
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
           }
}

function gen_cars(N) {
    const cars = [];
    for (let i = 0; i < N; i++) {
        cars.push(Car.ai_controlled(road.lane_center(1), 100, 30, 50, i));
    }
    return cars; 
}

function save() {
    console.log("saving brain");
    localStorage.setItem("bestBrain", bestCar.serialize_brain());
}

function discard() {
    console.log("discarding brain");
    localStorage.removeItem("bestBrain");
}