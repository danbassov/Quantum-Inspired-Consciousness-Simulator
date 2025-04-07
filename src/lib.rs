use pyo3::prelude::*;
use crate::quantum_network::QuantumNeuralNetwork;

mod quantum_network;
mod quantum_neuron;
mod visualization;

// Add this to expose PyQuantumNeuralNetwork
#[pyclass]
pub struct PyQuantumNeuralNetwork {
    network: QuantumNeuralNetwork,
}

#[pymethods]
impl PyQuantumNeuralNetwork {
    #[new]
    fn new(num_neurons: usize) -> Self {
        PyQuantumNeuralNetwork {
            network: QuantumNeuralNetwork::new(num_neurons),
        }
    }

    fn decide(&mut self) -> f64 {
        self.network.decide()
    }
}

// Add this to expose Visualization
#[pyclass]
pub struct Visualization {
    size: usize,
}

#[pymethods]
impl Visualization {
    #[new]
    pub fn new(size: usize) -> Self {
        Visualization { size }
    }

    pub fn plot_decisions(&self) -> PyResult<()> {
        Python::with_gil(|py| {
            let plt = py.import("matplotlib.pyplot")?;
            plt.call_method0("show")?;
            Ok(())
        })
    }
}

#[pymodule]
fn quantum_consciousness(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyQuantumNeuralNetwork>()?;
    m.add_class::<Visualization>()?;  // Add this line
    Ok(())
}