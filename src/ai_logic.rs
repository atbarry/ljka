use bevy::prelude::*;
use ndarray::{Array2, Array1};
use ndarray_rand::{RandomExt, rand_distr::Uniform};
use rand::{thread_rng, Rng};

use crate::{MOVE_SPEED, NETWORK_LAYERS};

pub fn move_ai(query: &mut Query<(&mut Transform, &AI)>) {
    // move in a random direction for now
    for (mut transform, ai) in query.iter_mut() {
        let input = Array1::from(vec![transform.translation.x, transform.translation.y]);
        let output = ai.brain.feed_forward(input);
        let x = (output[0] * 2.  - 1.0) * MOVE_SPEED;
        let y = (output[1] * 2.  - 1.0) * MOVE_SPEED;
        transform.translation += Vec3::new(x, y, 0.0);
    }
}

#[derive(Component, Clone)]
pub struct AI {
    pub brain: NeuralNetwork,
}

impl AI {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut brain = NeuralNetwork::new(2, NETWORK_LAYERS.to_vec());

        Self {
            brain,
        }
    }

    pub fn learn_reproduce(&self, learn_rate: f32) -> Self {
            // create a function that mutates the weights and biases slightly

        let rng = &mut thread_rng();
        let mut apply_mutation = |x: f32| -> f32 {
            x + (rand::random::<f32>() - 0.5) * 2.0 * learn_rate
        };

        let mut layers = Vec::new();
        for layer in &self.brain.layers{
            let weights = layer.weights.mapv(&mut apply_mutation);
            let biases = layer.biases.mapv(&mut apply_mutation);
            layers.push(Layer{weights, biases});
        }

        Self {
            brain: NeuralNetwork{layers},
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
} 

#[derive(Debug, Clone)]
struct Layer {
    weights: Array2<f32>,
    biases: Array1<f32>,
}

impl NeuralNetwork {
    pub fn default() -> Self {
        Self::new(2, vec![2, 2])
    }

    fn new(num_inputs: usize, nodes_per_layer: Vec<usize>) -> Self {
        let mut layers = Vec::new();

        let layer_0 = Layer{
            weights: create_weights(num_inputs, nodes_per_layer[0]),
            biases: create_biases(nodes_per_layer[0]),
        };
        layers.push(layer_0);

        for i in 0..nodes_per_layer.len() - 1 {
            let layer = Layer{
                weights: create_weights(nodes_per_layer[i], nodes_per_layer[i + 1]),
                biases: create_biases(nodes_per_layer[i + 1]),
            };
            layers.push(layer);
        }
    
        Self {
           layers,
        }
    }

    fn feed_forward(&self, input: Array1<f32>) -> Array1<f32> {
        let mut output = input;
        for layer in &self.layers {
            output = layer.weights.dot(&output) + &layer.biases;
            output = output.mapv(sigmoid);
        }
        output
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn create_biases(size: usize) -> Array1<f32> {
    // create a vector of random values between -1 and 1
    let biases = Array1::random(size, Uniform::new(-1.0, 1.0));
    biases
}

fn create_weights(input_nodes: usize, output_nodes: usize) -> Array2<f32> {
    // create a matrix of random values between -1 and 1
    let weights = Array2::random((output_nodes, input_nodes), Uniform::new(-1.0, 1.0));
    weights
}

