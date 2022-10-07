from scipy.special import erf
import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm


def main():
    D = 1e-11  # m^2/s
    c_0 = 1
    c = lambda x, t: c_0 / 2 - c_0 / 2 * erf(x / np.sqrt(4 * D * t))
    x = np.linspace(-1, 1, 1000) * 1e-6
    t = np.linspace(0, 5, 1000)
    x, t = np.meshgrid(x, t)
    cs = c(x, t)
    fig = plt.figure()
    ax = fig.add_subplot(projection="3d")
    ax.set_xlabel("$x(\mu m)$")
    ax.set_ylabel("$t(s)$")
    ax.set_zlabel("$c(mol??)$")
    surf = ax.plot_surface(x, t, cs, cmap=cm.viridis)
    fig.colorbar(surf)
    plt.show()


if __name__ == "__main__":
    main()
