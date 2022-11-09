import discretisedfield as df
import matplotlib.pyplot as plt

fig, ax = plt.subplots()
df.plotting.mpl_field.add_colorwheel(ax, width=100, height=100, loc="center")
ax.axis("off")
fig.savefig(
    "colorwheel.png", dpi=100, bbox_inches="tight", pad_inches=0, transparent=True
)
