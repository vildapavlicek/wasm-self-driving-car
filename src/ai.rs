use itertools::Itertools;
use js_sys::Math::random;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::lerp;

#[wasm_bindgen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NeuralNetwork(#[wasm_bindgen(skip)] pub Vec<Level>);

impl NeuralNetwork {
    pub fn new(neuron_counts: &[usize]) -> Self {
        let mut levels = Vec::with_capacity(neuron_counts.len() - 1);
        for (input_count, output_count) in neuron_counts.iter().tuple_windows() {
            levels.push(Level::new(*input_count, *output_count));
        }

        Self(levels)
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        let mut outputs = self
            .0
            .first_mut()
            .expect("no neural network provided")
            .feed_forward(&inputs);

        for level in self.0.iter_mut().skip(1) {
            outputs = level.feed_forward(&outputs);
        }

        outputs
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
    pub outputs: Vec<f64>,
    pub biases: Vec<f64>,
    pub weights: Vec<Vec<f64>>,
}

impl Level {
    pub fn new(input_count: usize, output_count: usize) -> Self {
        Level {
            inputs: vec![0.; input_count],
            outputs: vec![0.; output_count],
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
            for _ in self.outputs.iter() {
                weights.push(random() * 2.0 - 1.0);
            }
            self.weights.push(weights);
        }

        for _ in 0..self.outputs.len() {
            self.biases.push(random() * 2.0 - 1.0);
        }

        self
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        self.inputs
            .iter_mut()
            .zip(inputs.iter())
            .for_each(|(old_input, new_input)| *old_input = *new_input);

        for (i, output) in self.outputs.iter_mut().enumerate() {
            let mut sum = 0.;
            for (j, _) in self.inputs.iter().enumerate() {
                sum += self
                    .inputs
                    .get(j)
                    .expect("expected input value, but none was found")
                    * self
                        .weights
                        .get(j)
                        .and_then(|w| w.get(i).copied())
                        .expect("expected weights value but none was found");
            }

            *output = match self.biases.get(i) {
                //Some(bias) => (sum + bias).clamp(-1., 1.),
                Some(b) if sum + *b > 0. => 1.,
                Some(b) if sum < *b => 0.,
                /* Some(b) if sum > *b => 1.,
                Some(b) if sum < *b => 0., */
                _ => 0.,
            }
        }

        self.outputs.clone()
    }
}
