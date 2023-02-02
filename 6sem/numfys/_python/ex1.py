import smumerix
import matplotlib.pyplot as plt
import json
from pathlib import Path
from time import time

curdir = Path(__file__).parent

if True:
    edg = smumerix.EventDrivenGas.new_uniform_v(5000, 0.04, 0.003)

    initial_speed_dist = edg.get_speeds()

    with open(curdir / "initial_speed.json", "w") as file:
        json.dump(initial_speed_dist, file)

    tic = time()
    edg.step_many(500_000)

    speeds = edg.get_speeds()
    for _ in range(10):
        edg.step_many(10_000)
        speeds += edg.get_speeds()
    toc = time()
    print(f"Simulation took {toc - tic} seconds")

    with open(curdir / "final_speed.json", "w") as file:
        json.dump(speeds, file)
else:
    with open(curdir / "initial_speed.json", "r") as file:
        initial_speed_dist = json.load(file)
    with open(curdir / "final_speed.json", "r") as file:
        speeds = json.load(file)

fig, (ax1, ax2) = plt.subplots(1, 2)

ax1.hist(initial_speed_dist, bins=[n * 0.01 + 0.005 for n in range(0, 8)])
ax1.set_title("Initial speed distribution")
ax1.set_xlabel("Speed")
ax1.set_ylabel("Amount of particles")

ax2.hist(speeds, 200)
ax2.set_title("Final speed distribution")
ax2.set_xlabel("Speed")

fig.savefig(curdir / "speed_dist_5000p_500000steps.png")

plt.show()
