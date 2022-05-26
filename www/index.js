import * as wasm from "hello-wasm-pack";
 // import * as self_driving_car from "wasm-self-driving-car";
 import { Car, KeyEvent, Road, Border, lerp } from "wasm-self-driving-car";
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


addKeyboardListeners();
animate();




 function animate() {
    car.update();

     canvas.height = window.innerHeight;

     ctx.save();
     ctx.translate(0, -car.y() + canvas.height * 0.7);
     draw_road(ctx, road);
     draw_car(ctx, car);
     ctx.restore();

     requestAnimationFrame(animate);
 }


/* function draw_car() {
        ctx.save();
        ctx.translate(car.x(), car.y());
        ctx.rotate(-car.angle());

        ctx.beginPath();
        ctx.rect(-car.width() / 2, -car.height() / 2, car.width(), car.height());
        ctx.fill();
        ctx.restore();
}

function draw_road() {
    ctx.lineWidth = 5;
        ctx.strokeStyle = "white";

        for (let i = 1; i <= road.lane_count() - 1; i++) {
            const x = lerp(road.left(), road.right(), i / road.lane_count());

            ctx.setLineDash([20, 20]);

            ctx.beginPath();
            ctx.moveTo(x, road.top());
            ctx.lineTo(x, road.bottom());
            ctx.stroke();
        }

         ctx.setLineDash([]);
         road.borders().forEach(border => {
            ctx.beginPath();
            ctx.moveTo(border.top_x(), border.top_y());
            ctx.lineTo(border.bottom_x(), border.bottom_y());
            ctx.stroke();
         })
} */


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

/* import { Car, Direction } from "wasm-self-driving-car"; */

// const canvas = document.getElementById("myCanvas");
// canvas.width = 200;
//
// const ctx = canvas.getContext("2d");
// const road = new Road(canvas.width / 2, canvas.width * 0.9);
// const car = new Car(road.getLaneCenter(1), 100, 30, 50);
//
// animate();
//
//
// function animate() {
//     car.update();
//
//     canvas.height = window.innerHeight;
//
//     ctx.save();
//     ctx.translate(0, -car.y + canvas.height * 0.7);
//     road.draw(ctx);
//     car.draw(ctx);
//
//     ctx.restore();
//     requestAnimationFrame(animate);
// }
/* 
const canvas = document.getElementById("myCanvas");
canvas.width = 200;

const ctx = canvas.getContext("2d");
const road = new Road(canvas.width / 2, canvas.width * 0.9);
const car = Car.new(road.getLaneCenter(1), 100, 30, 50);

animate();


function animate() {
    car.update();

    canvas.height = window.innerHeight;

    ctx.save();
    ctx.translate(0, -car.y + canvas.height * 0.7);
    road.draw(ctx);
    car.draw(ctx);

    ctx.restore();
    requestAnimationFrame(animate);
} */