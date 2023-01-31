from matplotlib import animation
from matplotlib.patches import Circle
from matplotlib.collections import PatchCollection
from smumerix import EventDrivenGas
import smumerix
import matplotlib.pyplot as plt
import numpy as np


def plot(edg: EventDrivenGas, ax: plt.Axes):
    x, y, sizes = edg.get_positions_sizes()
    patches = []
    for x1, y1, r in zip(x, y, sizes):
        patches.append(Circle((x1, y1), r))
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.set_aspect("equal")
    ax.add_collection(PatchCollection(patches))


fig = plt.figure()
ax = fig.gca()

edg = smumerix.EventDrivenGas.new_uniform_v(5, 0.04, 0.13)


def animate(frame):
    ...


anim = animation.FuncAnimation(fig, animate)
