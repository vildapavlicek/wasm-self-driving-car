//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

use wasm_self_driving_car::ai::{NeuralNetwork, *};
#[wasm_bindgen_test]
fn level_feed_forward_test() {
    let mut level = Level::new(2, 2);
    level.feed_forward(&vec![1., 2.]);
    assert_eq!(level.inputs, vec![1., 2.]);
}

#[wasm_bindgen_test]
fn test_new_nn() {
    let nn = NeuralNetwork::new(&[2, 2, 2]);
    assert_eq!(nn.0.len(), 2);

    let nn = NeuralNetwork::new(&[2, 2]);
    assert_eq!(nn.0.len(), 1);

    let nn = NeuralNetwork::new(&[2, 2, 2, 3]);
    assert_eq!(nn.0.len(), 3);
}

/* #[wasm_bindgen_test]
fn test_feed_forward_nn() {
    let mut nn = NeuralNetwork::new(&[2, 2, 2]);
    let outputs = nn.feed_forward(vec![1., 2.]);
    assert_eq!(outputs, vec![0., 0.]);
}
 */
