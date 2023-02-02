import smumerix
import matplotlib.pyplot as plt
import json
from pathlib import Path
from time import time
import numpy as np

curdir = Path(__file__).parent
edg = smumerix.EventDrivenGas.new_uniform_v_different_m(5000, 0.04, 0.003)
masses = edg.get_masses()

if False:
    initial_speed_dist = edg.get_speeds()

    with open(curdir / "cache" / "initial_speed_2.json", "w") as file:
        json.dump(initial_speed_dist, file)

    tic = time()
    edg.step_many(5_000_000)

    speeds = edg.get_speeds()
    light_speeds = [speed for (speed, mass) in zip(speeds, masses) if mass < 2.0]
    heavy_speeds = [speed for (speed, mass) in zip(speeds, masses) if mass > 2.0]

    for _ in range(10):
        edg.step_many(500_000)
        speeds = edg.get_speeds()
        light_speeds += [speed for (speed, mass) in zip(speeds, masses) if mass < 2.0]
        heavy_speeds += [speed for (speed, mass) in zip(speeds, masses) if mass > 2.0]
    toc = time()
    print(f"Simulation took {toc - tic:4f} seconds")

    with open(curdir / "cache" / "final_speed_light_2.json", "w") as file:
        json.dump(light_speeds, file)
    with open(curdir / "cache" / "final_speed_heavy_2.json", "w") as file:
        json.dump(heavy_speeds, file)
else:
    with open(curdir / "cache" / "initial_speed_2.json", "r") as file:
        initial_speed_dist = json.load(file)
    with open(curdir / "cache" / "final_speed_light_2.json", "r") as file:
        light_speeds = json.load(file)
    with open(curdir / "cache" / "final_speed_heavy_2.json", "r") as file:
        heavy_speeds = json.load(file)


fig, ((ax_1_init, ax_1_after), (ax_2_init, ax_2_after)) = plt.subplots(2, 2)
ax_1_after.get_shared_x_axes().join(ax_1_after, ax_2_after)


initial_speed_light = [
    speed for (speed, mass) in zip(initial_speed_dist, masses) if mass < 2.0
]
initial_speed_heavy = [
    speed for (speed, mass) in zip(initial_speed_dist, masses) if mass > 2.0
]
avg_speed_light = np.average(light_speeds)
avg_speed_heavy = np.average(heavy_speeds)

print(f"Average speed of light particles: {avg_speed_light}")
print(f"Average speed of heavy particles: {avg_speed_heavy}")

ax_1_init.hist(initial_speed_light, bins=[n * 0.01 + 0.005 for n in range(0, 8)])
ax_1_init.set_title("Initial speed $m=m_0$")
ax_1_init.set_xlabel("Speed")
ax_1_init.set_ylabel("Amount of particles")

ax_2_init.hist(initial_speed_heavy, bins=[n * 0.01 + 0.005 for n in range(0, 8)])
ax_2_init.set_title("Initial speed $m=4m_0$")
ax_2_init.set_xlabel("Speed")
ax_2_init.set_ylabel("Amount of particles")

ax_1_after.hist(light_speeds, 200, label="Simulated")
ax_1_after.set_title("Final speed $m=m_0$")
ax_1_after.set_xlabel("Speed")
ax_1_after.legend()

ax_2_after.hist(heavy_speeds, 200, label="Simulated")
ax_2_after.set_title("Final speed $m=4m_0$")
ax_2_after.set_xlabel("Speed")
ax_2_after.legend()


fig.savefig(curdir / "2_masses_5000p_5000000steps.png")

plt.show()
