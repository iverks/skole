import discretisedfield as df

import matplotlib.pyplot as plt
import numpy as np

from mpl_toolkits.axes_grid1.anchored_artists import AnchoredSizeBar
import matplotlib.font_manager as fm
import matplotlib.patheffects as patheffects

read_field = df.Field.fromfile("magnetization.ovf")


def add_scalebar(ax: plt.Axes):
    scalebar_kwargs = {
        "size": 5,
        "label": "5 $\mathrm{\mu}$m",
        "loc": 4,
        "frameon": False,
        "color": "white",
        "size_vertical": 0.2,
        "label_top": False,
        "fontproperties": fm.FontProperties(size=18),
    }
    scalebar = AnchoredSizeBar(transform=ax.transData, **scalebar_kwargs)
    # Denne legger til et svart omriss rundt scalebar teksten, for å gjøre den lettere å lese
    scalebar.txt_label._text.set_path_effects(
        [patheffects.withStroke(linewidth=2, foreground="black", capstyle="round")]
    )
    ax.add_artist(scalebar)


zlims = (read_field.mesh.region.p1[2], read_field.mesh.region.p2[2])
magnet_w = (
    read_field.mesh.region.p2[0] - read_field.mesh.region.p1[0]
) / 8  # assumes symmetric
magnet_x = 6
magnet_y = 6
rm_margin = 0.15 * magnet_w
subregion = df.Region(
    p1=(magnet_w * magnet_x + rm_margin, magnet_w * magnet_y + rm_margin, zlims[0]),
    p2=(
        magnet_w * (magnet_x + 1) - rm_margin,
        magnet_w * (magnet_y + 1) - rm_margin,
        zlims[1],
    ),
)

fig, ax = plt.subplots()
read_field.plane("z").mpl.lightness(
    ax=ax,
    cmap="turbo",
    lightness_field=read_field.z,
    colorwheel_args={"loc": 1},
)

# ax.annotate(
#     "a",
#     xy=(0.075, 0.85),
#     xycoords="axes fraction",
#     fontsize=22,
#     color="white",
#     path_effects=[
#         patheffects.withStroke(linewidth=2, foreground="black", capstyle="round")
#     ],
# )

add_scalebar(ax)
ax.axis("off")
ax.set_facecolor("grey")
fig.savefig("all_horseshoe_sim.png", bbox_inches="tight", pad_inches=0, dpi=300)
