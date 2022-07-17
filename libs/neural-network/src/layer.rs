use crate::neuron::Neuron;
use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

#[derive(Clone, Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .fold(vec![1f32; self.neurons.len()], |mut outputs, neuron| {
                outputs.push(neuron.propagate(&inputs));
                outputs
            })
    }

    pub fn random(rng: &mut dyn RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Self { neurons }
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self::new(neurons)
    }
}
