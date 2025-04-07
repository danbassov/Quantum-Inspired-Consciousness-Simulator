use pyo3::prelude::*;
use crate::quantum_network::QuantumNeuralNetwork;

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

  pub fn decide(&self) -> f64 {
    self.network.decide()
  }
}

#[pymodule]
fn quantum_consciousness(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<PyQuantumNeuralNetwork>()?;
  Ok(())
}