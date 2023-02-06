import smumerix
import matplotlib.pyplot as plt
import json
from pathlib import Path
from time import time
import numpy as np

num_particles = 500
light_mass = 1.0
heavy_mass = 4.0

for xi in [1, 0.9, 0.8]:
    curdir = Path(__file__).parent
    edg = smumerix.EventDrivenGas.new_uniform_v_different_m(
        num_particles, 0.04, 0.003, xi
    )
    masses = edg.get_masses()
    light_es = []
    heavy_es = []

    for i in range(200):
        edg.step_many(num_particles // 8)
        speeds = edg.get_speeds()
        light_speeds = np.array(
            [speed for (speed, mass) in zip(speeds, masses) if mass < 2.0]
        )
        light_avg_e = light_mass / 2 * np.average(light_speeds) ** 2
        heavy_speeds = np.array(
            [speed for (speed, mass) in zip(speeds, masses) if mass > 2.0]
        )
        heavy_avg_e = heavy_mass / 2 * np.average(heavy_speeds) ** 2

        light_es.append(light_avg_e)
        heavy_es.append(heavy_avg_e)

    plt.plot(light_es)
    plt.plot(heavy_es)
    plt.show()
