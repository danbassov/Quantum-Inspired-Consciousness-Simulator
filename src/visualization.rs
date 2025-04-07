use pyo3::prelude::*;
use crate::PyQuantumNeuralNetwork; // Import the PyQuantumNeuralNetwork struct from lib.rs

#[pyclass]
pub struct Visualization {
    network: PyQuantumNeuralNetwork, // Use the existing PyQuantumNeuralNetwork struct
}

#[pymethods]
impl Visualization {
    #[new]
    pub fn new(num_neurons: usize) -> Self {
        Visualization {
            network: PyQuantumNeuralNetwork::new(num_neurons),
        }
    }

    pub fn plot_decisions(&mut self) -> PyResult<()> {
        // Simulate 100 decisions
        let decisions: Vec<f64> = (0..100).map(|_| self.network.decide()).collect();

        // Plot the decisions (this requires matplotlib in Python)
        Python::with_gil(|py| {
            let plt = py.import("matplotlib.pyplot")?;
            plt.call_method1("plot", (decisions,))?;
            plt.call_method0("show")?;
            Ok(())
        })
    }
}

#[pymodule]
fn visualization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Visualization>()?; // Expose the Visualization struct to Python
    Ok(())
}