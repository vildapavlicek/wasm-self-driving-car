/* import {Car, Road, lerp, Sensor} from "wasm-self-driving-car";

export function draw_car(ctx, car) {
    ctx.save();
    ctx.translate(car.x(), car.y());
    ctx.rotate(-car.angle());

    ctx.beginPath();
    ctx.rect(-car.width() / 2, -car.height() / 2, car.width(), car.height());
    ctx.fill();
    ctx.restore();
}

export function draw_road(ctx, road) {
ctx.lineWidth = 5;
    ctx.strokeStyle = "white";

    for (let i = 1; i <= road.lane_count() - 1; i++) {
        const x = lerp(road.left(), road.right(), i / road.lane_count());
        console.log(x);
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

/* export function draw_sensor_rays(ctx, sensor, x, y) {
    for (let i = 0; i < sensor.rays_count(); i++) {
        ctx.beginPath();
        ctx.lineWidth = 2;
        ctx.strokeStyle = "yellow";

        let ray = sensor.ray(i);

        ctx.moveTo(ray.x(), ray.y());
    }
} */