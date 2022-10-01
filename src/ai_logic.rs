use bevy::prelude::*;
use ndarray::{Array2, Array1};
use ndarray_rand::{RandomExt, rand_distr::Uniform};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::target::Target;

#[derive(Component)]
pub struct Ai {
    brain: NeuralNetwork,
}

impl Ai{
    
}

pub struct GenePool{
    genes: Vec<NeuralNetwork>,
    population: u32,
}

impl GenePool{
    pub fn new(population: u32) -> Self{
        let mut genes = Vec::new();
        for _ in 0..population{
            genes.push(NeuralNetwork::default());
        }

        Self{
            genes,
            population,
        }
    }

    fn add_genes(&mut self, genes: Vec<NeuralNetwork>){
        // randomly select genes to replace
        self.genes.shuffle(&mut thread_rng());
        self.genes.truncate(self.genes.len() - genes.len());

        // Add new genes to the pool
        self.genes.extend(genes);

        // Mutate the genes
        for gene in self.genes.iter_mut(){
            gene.mutate_layers();
        }
    }

    pub fn add_successful_ai(
        &mut self,
        ai_query: &Query<(Entity,&Transform, &Ai)>,
        target_query: &Query<&Target>,
    ) {
        let mut genes = Vec::new();
        let mut success_num = 0;

        for (_, transform, ai) in ai_query.iter() {
            for target in target_query.iter() {
                let pos = transform.translation;

                // if the ai is in the target, add the gene to the pool
                if target.is_in_target(Vec2::new(pos.x, pos.y)) {
                    genes.push(ai.brain.clone());
                    success_num += 1;
                }
            }
        }

        println!("Success: {}", success_num);
        self.add_genes(genes);
    }

    pub fn create_new_ai(&self) -> Ai{
        let mut rng = thread_rng();
        let gene = self.genes.choose(&mut rng).unwrap();
        Ai{
            brain: gene.clone(),
        }
    }

}

#[derive(Debug, Clone)]
struct NeuralNetwork {
    layers: Vec<Layer>,
} 

#[derive(Debug, Clone)]
struct Layer {
    weights: Array2<f32>,
    biases: Array1<f32>,
}

impl NeuralNetwork {
    pub fn default() -> Self {
        Self::new(2, vec![3, 3, 2])
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

    // create a function that mutates the weights and biases slightly
    fn mutate_layers(&self) -> Self {
        let mut layers = Vec::new();
        for layer in &self.layers {
            let weights = layer.weights.mapv(mutate);
            let biases = layer.biases.mapv(mutate);
            layers.push(Layer{weights, biases});
        }
        Self {
            layers,
        }
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn mutate(x: f32) -> f32 {
    x + (rand::random::<f32>() - 0.5) * 0.05
}

fn create_biases(size: usize) -> Array1<f32> {
    // create a vector of random values between -1 and 1
    let mut biases = Array1::random(size, Uniform::new(-1.0, 1.0));
    
    biases
}

fn create_weights(input_nodes: usize, output_nodes: usize) -> Array2<f32> {
    // create a matrix of random values between -1 and 1
    let mut weights = Array2::random((output_nodes, input_nodes), Uniform::new(-1.0, 1.0));
    weights
}