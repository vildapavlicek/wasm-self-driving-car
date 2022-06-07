import * as wasm from "hello-wasm-pack";
 // import * as self_driving_car from "wasm-self-driving-car";
 import { Car, KeyEvent, Road, Border, Traffic, Level } from "wasm-self-driving-car";

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
const car = Car.ai_controlled(road.lane_center(1), 100, 30, 50);
// other cars
const traffic = Traffic.new();

// traffic.add(Car.no_control(road.lane_center(2), -100, 30, 50, 0));
traffic.add(Car.no_control(road.lane_center(1), -100, 30, 50, 2));
// traffic.add(Car.no_control(road.lane_center(0), -100, 30, 50, 0));

addKeyboardListeners();
animate();


function animate() {

    traffic.update(road);

    car.update(road, traffic);

    carCanvas.height = window.innerHeight;
    networkCanvas.height = window.innerHeight;

    carCtx.save();
    carCtx.translate(0, -car.y + carCanvas.height * 0.7);
    road.draw(carCtx);

    traffic.draw(carCtx);

    car.draw(carCtx);
    carCtx.restore();

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