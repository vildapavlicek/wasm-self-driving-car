use itertools::Itertools;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

pub type Boarders = [((f64, f64), (f64, f64))];

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct IntersectionPoint {
    pub x: f64,
    pub y: f64,
    pub offset: f64,
}

pub fn get_intersection(
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
    d: (f64, f64),
) -> Option<IntersectionPoint> {
    let t_top = (d.0 - c.0) * (a.1 - c.1) - (d.1 - c.1) * (a.0 - c.0);
    let u_top = (c.1 - a.1) * (a.0 - b.0) - (c.0 - a.0) * (a.1 - b.1);
    let bottom = (d.1 - c.1) * (b.0 - a.0) - (d.0 - c.0) * (b.1 - a.1);

    match bottom.ne(&0.) {
        false => None,
        true => {
            let t = t_top / bottom;
            let u = u_top / bottom;

            if (0. ..=1.).contains(&t) && (0. ..=1.).contains(&u) {
                let x = lerp(a.0, b.0, t);
                let y = lerp(a.1, b.1, t);
                let offset = t;

                return Some(IntersectionPoint { x, y, offset });
            };

            None
        }
    }
}

pub fn poly_intersection_with_borders(poly1: &[(f64, f64)], borders: &Boarders) -> bool {
    for (polygon_1, polygon_2) in poly1.iter().circular_tuple_windows() {
        for border in borders {
            if get_intersection(*polygon_1, *polygon_2, border.0, border.1).is_some() {
                return true;
            }
        }
    }
    false
}

pub fn poly_intersection_with_poly(poly1: &[(f64, f64)], poly2: &[(f64, f64)]) -> bool {
    for (polygon_1, polygon_2) in poly1.iter().circular_tuple_windows() {
        for (polygon_3, polygon_4) in poly2.iter().circular_tuple_windows() {
            if get_intersection(*polygon_1, *polygon_2, *polygon_3, *polygon_4).is_some() {
                return true;
            }
        }
    }
    false
}
