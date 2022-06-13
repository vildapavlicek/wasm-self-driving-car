use crate::ai::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const MARGIN: u32 = 50;
const LEFT: u32 = MARGIN;
const TOP: u32 = MARGIN;
const ARROWS: [&str; 4] = ["🠉", "🠈", "🠊", "🠋"];
const NODE_RADIUS: f64 = 18.;

#[wasm_bindgen]
pub struct Visualizer;

#[wasm_bindgen]
impl Visualizer {
    pub fn draw_network(ctx: &CanvasRenderingContext2d, network: &NeuralNetwork) {
        let width = ctx.canvas().unwrap().width() - MARGIN * 2;
        let height = ctx.canvas().unwrap().height() - MARGIN * 2;

        let level_height = height as f64 / network.0.len() as f64;

        for (i, _) in network.0.iter().enumerate().rev() {
            let level_top = TOP as f64
                + crate::utils::lerp(
                    height as f64 - level_height,
                    0_f64,
                    match network.0.len() {
                        1 => 0.5,
                        _ => i as f64 / (network.0.len() - 1) as f64,
                    },
                );

            let array = js_sys::Array::new();
            array.push(&JsValue::from(7));
            array.push(&JsValue::from(3));
            ctx.set_line_dash(&array)
                .expect("failed to set line dash while drawing network");

            draw_level(
                &ctx,
                network.0.get(i).expect("expected level, got nothing"),
                // LEFT,
                level_top,
                width,
                level_height,
                match i {
                    _ if i == network.0.len() - 1 => Some(ARROWS),
                    _ => None,
                },
            )
        }
    }
}

fn draw_level(
    ctx: &CanvasRenderingContext2d,
    level: &Level,
    //left: u32,
    top: f64,
    width: u32,
    height: f64,
    icons: Option<[&str; 4]>,
) {
    let right = LEFT + width;
    let bottom = top + height;

    for (i, _) in level.inputs.iter().enumerate() {
        for (j, _) in level.outputs.borrow().iter().enumerate() {
            ctx.begin_path();
            ctx.move_to(get_node(level.inputs.len(), i, right), bottom);
            ctx.line_to(get_node(level.outputs.borrow().len(), j, right), top);

            ctx.set_line_width(2.);
            ctx.set_stroke_style(&crate::utils::get_rgba(level.weights[i][j]));
            ctx.stroke();
        }
    }

    for (i, input) in level.inputs.iter().enumerate() {
        let x = get_node(level.inputs.len(), i, right);
        ctx.begin_path();
        ctx.arc(x, bottom, NODE_RADIUS, 0., 2. * std::f64::consts::PI)
            .expect("failed to `arc`");
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill();
        ctx.begin_path();
        ctx.arc(x, bottom, NODE_RADIUS * 0.6, 0., 2. * std::f64::consts::PI)
            .expect("failed to `arc`");
        ctx.set_fill_style(&crate::utils::get_rgba(*input));
        ctx.fill();
    }

    for (i, output) in level.outputs.borrow().iter().enumerate() {
        let x = get_node(level.outputs.borrow().len(), i, right);
        ctx.begin_path();
        ctx.arc(x, top, NODE_RADIUS, 0., 2. * std::f64::consts::PI)
            .expect("failed to `arc`");
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill();
        ctx.begin_path();
        ctx.arc(x, top, NODE_RADIUS * 0.6, 0., 2. * std::f64::consts::PI)
            .expect("failed to `arc`");
        ctx.set_fill_style(&crate::utils::get_rgba(*output));
        ctx.fill();

        ctx.begin_path();
        ctx.set_line_width(2.);
        ctx.arc(x, top, NODE_RADIUS * 0.8, 0., 2. * std::f64::consts::PI)
            .expect("failed to `arc`");
        ctx.set_stroke_style(&crate::utils::get_rgba(*level.biases.get(i).unwrap()));

        let line_dash = js_sys::Array::new();
        line_dash.push(&JsValue::from(3));
        line_dash.push(&JsValue::from(3));

        ctx.set_line_dash(&line_dash)
            .expect("failed to `set_line_dash`");
        ctx.stroke();
        ctx.set_line_dash(&js_sys::Array::new())
            .expect("failed to `set_line_dash`");

        if let Some(icons) = icons {
            ctx.begin_path();
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style(&JsValue::from("black"));
            ctx.set_stroke_style(&JsValue::from("white"));
            ctx.set_font(format!("{}px Arial", NODE_RADIUS * 1.5).as_str());
            ctx.fill_text(icons[i], x as f64, top + NODE_RADIUS * 0.1)
                .expect("failed to `fill_text`");
            ctx.set_line_width(0.5);
            ctx.stroke_text(icons[i], x as f64, top + NODE_RADIUS * 0.1)
                .expect("failed to `fill_text`");
        }
    }
}

fn get_node(nodes_len: usize, index: usize, right: u32) -> f64 {
    crate::utils::lerp(
        LEFT as f64,
        right as f64,
        match nodes_len {
            1 => 0.5,
            _ => index as f64 / (nodes_len - 1) as f64,
        },
    )
}
