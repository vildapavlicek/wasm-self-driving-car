use crate::log;
use itertools::Itertools;
use js_sys::Math::random;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
pub struct NeuralNetwork(pub Vec<Level>);

impl NeuralNetwork {
    pub fn new(neuron_counts: &[usize]) -> Self {
        let mut levels = Vec::with_capacity(neuron_counts.len() - 1);
        for (input_count, output_count) in neuron_counts.iter().tuple_windows() {
            levels.push(Level::new(*input_count, *output_count));
        }

        Self(levels)
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        log!("neural netwrok input: {inputs:?}");
        let mut outputs = self
            .0
            .first_mut()
            .expect("no neural network provided")
            .feed_forward(&inputs);

        log!("neural network output: {outputs:?}");

        for level in self.0.iter_mut().skip(1) {
            outputs = level.feed_forward(&outputs);
            log!("neural network output: {outputs:?}");
        }

        outputs
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Level {
    #[wasm_bindgen(skip)]
    pub inputs: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub outputs: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub biases: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub weights: Vec<Vec<f64>>,
}

#[wasm_bindgen]
impl Level {
    pub fn new(input_count: usize, output_count: usize) -> Self {
        log!("creating new level; input count: {input_count}, output count: {output_count}",);
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

        //
        for _ in 0..self.outputs.len() {
            self.biases.push(random() * 2.0 - 1.0);
        }

        // log!("randomized self: {self:#?}");

        self
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        // log!("old inputs: {:?},\nnew inputs: {inputs:?}", self.inputs);

        self.inputs
            .iter_mut()
            .zip(inputs.iter())
            .for_each(|(old_input, new_input)| *old_input = *new_input);

        // log!("replaced old inputs: {:?}", self.inputs);

        // log!("outputs are {:?}", self.outputs);
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
                        .and_then(|w| w.get(i).map(|i| *i))
                        .expect("expected weights value but none was found");
            }

            *output = match self.biases.get(i) {
                Some(b) if sum > *b => {
                    //log!("sum {} bias {}, resolved to 1.", sum, b);
                    1.
                }
                Some(b) if sum < *b => {
                    //log!("sum {} bias {}, resolved to 0.", sum, b);
                    0.
                }
                _ => {
                    //log!("bias not found!");
                    0.
                }
            }
        }

        // log!("new outputs {:?}", self.outputs);

        self.outputs.clone()
    }
}

/* #[test]
fn level_feed_forward_test() {
    let mut level = Level::new(2, 2);
    level.feed_forward(vec![1., 2.]);
    assert_eq!(level.inputs, vec![1., 2.]);
}
 */
