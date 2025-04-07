use quantum_consciousness::quantum_network::QuantumNeuralNetwork;

fn main() {
    let num_neurons = 10;
    let mut network = QuantumNeuralNetwork::new(num_neurons);
    println!("Decision: {}", network.decide());
}