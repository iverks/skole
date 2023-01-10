import smumerix
import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit


def analytical(t: int, alpha: float):
    return t**-alpha


xs = np.arange(2, 100, dtype=int)

res = smumerix.preex.probability_distribution()
num_ys = res[2:100]
alpha = curve_fit(analytical, xs, num_ys)[0]

# plt.plot(num_ys)
# plt.plot(analytical(xs, alpha))
# plt.show()

print(alpha)  # approx 2
