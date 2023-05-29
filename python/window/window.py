
# dependencies
# pip install numpy
# pip install matplotlib
import numpy as np
import matplotlib.pyplot as plt

# Generate some sample data
data = np.random.random(100)

# Apply Hanning window
windowed_data = data * np.hanning(len(data))

# Plot original and windowed data
plt.figure()
plt.plot(data, label='Original Data')
plt.plot(windowed_data, label='Windowed Data (Hanning)')
plt.legend()
plt.show()
