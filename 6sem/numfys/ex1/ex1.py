import smumerix
import matplotlib.pyplot as plt
import json
from pathlib import Path
from time import time
import numpy as np

curdir = Path(__file__).parent
edg = smumerix.EventDrivenGas.new_uniform_v(5000, 0.04, 0.003)
m = edg.get_masses()[0]

if True:
    initial_speed_dist = edg.get_speeds()

    with open(curdir / "cache" / "initial_speed.json", "w") as file:
        json.dump(initial_speed_dist, file)

    tic = time()
    edg.step_many(5_000_000)

    speeds = edg.get_speeds()
    for _ in range(10):
        edg.step_many(500_000)
        speeds += edg.get_speeds()
    toc = time()
    print(f"Simulation took {toc - tic} seconds")

    with open(curdir / "cache" / "final_speed.json", "w") as file:
        json.dump(speeds, file)
else:
    with open(curdir / "cache" / "initial_speed.json", "r") as file:
        initial_speed_dist = json.load(file)
    with open(curdir / "cache" / "final_speed.json", "r") as file:
        speeds = json.load(file)


def analytic(v: np.ndarray):
    kT = 0.0008
    return m * v / kT * np.exp(-m * v**2 / (2 * kT))


v = np.linspace(0, 0.15)

fig, (ax1, ax2) = plt.subplots(1, 2)

ax1.hist(initial_speed_dist, bins=[n * 0.01 + 0.005 for n in range(0, 8)])
ax1.set_title("Initial speed distribution")
ax1.set_xlabel("Speed")
ax1.set_ylabel("Amount of particles")

ax2.hist(speeds, 200, label="Simulated")
ax2.plot(v, analytic(v) * 37, label="Analytical\n(fitted)")
ax2.set_title("Final speed distribution")
ax2.set_xlabel("Speed")
ax2.legend()

fig.savefig(curdir / "speed_dist_5000p_500000steps.png")

plt.show()
