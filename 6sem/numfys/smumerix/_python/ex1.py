import smumerix
import matplotlib.pyplot as plt
import numpy as np

edg = smumerix.EventDrivenGas.new_uniform_v(5, 0.04, 0.2)
xs, ys, sizes = edg.get_positions_sizes()

plt.scatter(xs, ys, np.array(sizes) ** 2 * np.pi)
plt.show()
