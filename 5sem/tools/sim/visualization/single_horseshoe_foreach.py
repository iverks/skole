import discretisedfield as df
import micromagneticmodel as mm

import oommfc as oc
import matplotlib.pyplot as plt
import numpy as np

from mpl_toolkits.axes_grid1.anchored_artists import AnchoredSizeBar
import matplotlib.font_manager as fm
import matplotlib.patheffects as patheffects

read_field = df.Field.fromfile("magnetization.ovf")


def add_scalebar(ax: plt.Axes):
    scalebar_kwargs = {
        "size": 0.5,
        "label": "500nm",
        "loc": 8,
        "frameon": False,
        "color": "black",
        "size_vertical": 0.02,
        "label_top": False,
        "fontproperties": fm.FontProperties(size=18),
    }
    scalebar = AnchoredSizeBar(transform=ax.transData, **scalebar_kwargs)
    # Denne legger til et svart omriss rundt scalebar teksten, for å gjøre den lettere å lese
    scalebar.txt_label._text.set_path_effects(
        [patheffects.withStroke(linewidth=2, foreground="black", capstyle="round")]
    )
    ax.add_artist(scalebar)


def do_da_thing(magnet_x: int, magnet_y: int):
    zlims = (read_field.mesh.region.p1[2], read_field.mesh.region.p2[2])
    magnet_w = (
        read_field.mesh.region.p2[0] - read_field.mesh.region.p1[0]
    ) / 8  # assumes symmetric
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
    read_field[subregion].plane("z").mpl.lightness(
        ax=ax,
        cmap="turbo",
        lightness_field=read_field[subregion].z,
        colorwheel_args={"loc": 1},
    )
    read_field[subregion].plane(z=10e-9, n=(30, 30)).mpl.vector(
        ax=ax, scale=30, colorbar=False, use_color=False
    )
    ax.annotate(
        f"{magnet_x}, {magnet_y}",
        xy=(0.075, 0.85),
        xycoords="axes fraction",
        fontsize=22,
        color="white",
        path_effects=[
            patheffects.withStroke(linewidth=2, foreground="black", capstyle="round")
        ],
    )

    add_scalebar(ax)
    ax.axis("off")
    fig.savefig(
        f"all_shoes/{magnet_x}_{magnet_y}.png", bbox_inches="tight", pad_inches=0
    )


for y in range(8):
    for x in range(8):
        do_da_thing(x, y)
