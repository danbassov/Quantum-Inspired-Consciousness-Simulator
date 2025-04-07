import quantum_consciousness
import matplotlib.pyplot as plt

# Network with 10 neurons
network = quantum_consciousness.PyQuantumNeuralNetwork(10)

# Simulate 100 decisions
decisions = [network.decide() for _ in range(100)]
print("Decisions:", decisions)

# Plot using matplotlib directly (simpler approach)
plt.plot(decisions)
plt.xlabel("Decision Number")
plt.ylabel("Decision Value")
plt.title("Quantum Consciousness Simulator")
plt.show()