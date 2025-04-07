import init, { QuantumNeuralNetwork } from './pkg/quantum_consciousness.js';

async function run() {
    await init();
    const network = QuantumNeuralNetwork.new(10);
    console.log(network.decide());
}

run();