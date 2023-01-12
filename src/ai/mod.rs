pub mod agents;

use js_sys::Math::random;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::lerp;

#[wasm_bindgen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NeuralNetwork(#[wasm_bindgen(skip)] pub Vec<Level>);

impl NeuralNetwork {
    pub fn new(neuron_counts: &[usize]) -> Self {
        let mut levels = Vec::with_capacity(neuron_counts.len() - 1);
        for (input_count_index, output_count) in neuron_counts.iter().skip(1).enumerate() {
            levels.push(Level::new(neuron_counts[input_count_index], *output_count));
        }

        Self(levels)
    }

    pub fn feed_forward_2(&self, inputs: Vec<f64>) {
        // SAFETY: each network is create with atleast input layer
        let first_level = unsafe { self.0.get_unchecked(0) };

        feed_forward(
            &RefCell::new(inputs),
            &first_level.outputs,
            &first_level.weights,
            &first_level.biases,
        );

        for (index, level) in self.0.iter().skip(1).enumerate() {
            feed_forward(
                &self.0[index].outputs,
                &level.outputs,
                &level.weights,
                &level.biases,
            )
        }
    }
}

#[wasm_bindgen]
impl NeuralNetwork {
    pub fn serialize_brain(&self) -> String {
        serde_json::to_string(&self).expect("failed to serialize brain")
    }

    pub fn deserialize_brain(json: String) -> Option<NeuralNetwork> {
        serde_json::from_str::<NeuralNetwork>(&json).ok()
    }

    pub fn mutate(&self, mutation_rate: f64) -> Self {
        let mut levels = self.0.clone();

        for level in levels.iter_mut() {
            for bias in level.biases.iter_mut() {
                *bias = lerp(*bias, random() * 2. - 1., mutation_rate);
            }

            for weight_vec in level.weights.iter_mut() {
                for weight in weight_vec.iter_mut() {
                    *weight = lerp(*weight, random() * 2. - 1., mutation_rate);
                }
            }
        }
        Self(levels)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Level {
    pub inputs: Vec<f64>,
    pub outputs: RefCell<Vec<f64>>,
    pub biases: Vec<f64>,
    pub weights: Vec<Vec<f64>>,
}

impl Level {
    pub fn new(input_count: usize, output_count: usize) -> Self {
        Level {
            inputs: vec![0.; input_count],
            outputs: RefCell::new(vec![0.; output_count]),
            biases: vec![],
            weights: vec![],
        }
        .randomize()
    }
}

impl Level {
    fn randomize(mut self) -> Self {
        // for each input we need to generate a random weights
        for _ in self.inputs.iter() {
            let mut weights = vec![];
            // we have one weight value for each output, from 1 input we map weights to each output
            for _ in self.outputs.borrow().iter() {
                weights.push(random() * 2.0 - 1.0);
            }
            self.weights.push(weights);
        }

        for _ in 0..self.outputs.borrow().len() {
            self.biases.push(random() * 2.0 - 1.0);
        }

        self
    }
}

pub fn feed_forward(
    inputs: &RefCell<Vec<f64>>,
    outputs: &RefCell<Vec<f64>>,
    weights: &[Vec<f64>],
    biases: &[f64],
) {
    for (i, output) in outputs.borrow_mut().iter_mut().enumerate() {
        let mut sum = 0.;
        for (j, input) in inputs.borrow().iter().enumerate() {
            sum += input * weights[j][i]
        }

        // SAFETY: we can't get out of bounds as the index is based on iterator
        *output = ((sum + unsafe { biases.get_unchecked(i) } > 0.) as u8) as f64;
    }
}
