import * as wasm from "hello-wasm-pack";
 // import * as self_driving_car from "wasm-self-driving-car";
 import { Car, KeyEvent, Road, Border, lerp, get_canvas, use_canvas } from "wasm-self-driving-car";
 import { draw_car, draw_road } from "./helpers";

 // initialize canvas
 const canvas = document.getElementById("myCanvas");
 canvas.width = 200;
 const ctx = canvas.getContext("2d");


// init road
// const road = new Road(canvas.width / 2, canvas.width * 0.9);
const road = Road.new(canvas.width / 2, canvas.width * 0.9, 3);
// get our car
const car = Car.new(road.lane_center(1), 100, 30, 50);
/* 
const my_canvas = get_canvas();
use_canvas(ctx); */


addKeyboardListeners();
animate();









 function animate() {
    car.update();

    canvas.height = window.innerHeight;

    ctx.save();
    ctx.translate(0, -car.y() + canvas.height * 0.7);
    road.draw(ctx);
    // draw_road(ctx, road);
    //draw_car(ctx, car);
    car.draw(ctx);
    ctx.restore();

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