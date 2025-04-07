use crate::quantum_neuron::QuantumNeuron;

pub struct QuantumNeuralNetwork {
    pub neurons: Vec<QuantumNeuron>,
}

impl QuantumNeuralNetwork {
    pub fn new(num_neurons: usize) -> Self {
        let mut neurons = Vec::with_capacity(num_neurons);
        for _ in 0..num_neurons {
            neurons.push(QuantumNeuron::new());
        }
        QuantumNeuralNetwork { neurons }
    }

    pub fn decide(&mut self) -> f64 {
        let sum: u8 = self.neurons.iter_mut()
            .map(|neuron| neuron.observe())
            .sum();
        sum as f64 / self.neurons.len() as f64
    }
}