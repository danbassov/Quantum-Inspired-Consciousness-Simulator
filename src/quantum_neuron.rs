use rand::Rng;
use pyo3::prelude::*;

#[pyclass]
pub struct QuantumNeuron {
    entangled_with: Option<usize>,
    state: f64, // Simplified for now
}

#[pymethods]
impl QuantumNeuron {
    #[new]
    pub fn new() -> Self {
        QuantumNeuron {
            entangled_with: None,
            state: 0.5, // Start with 50% probability
        }
    }

    pub fn observe(&mut self) -> u8 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.state { 1 } else { 0 }
    }
}