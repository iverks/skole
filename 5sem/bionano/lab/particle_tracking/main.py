from __future__ import (
    division,
    unicode_literals,
    print_function,
)
from functools import lru_cache  # for compatibility with Python 2 and 3

import matplotlib as mpl
import matplotlib.pyplot as plt

import numpy as np
import pandas as pd
from pandas import DataFrame, Series  # for convenience

import pims
import trackpy as tp

# Optionally, tweak styles.
mpl.rc("figure", figsize=(10, 5))
mpl.rc("image", cmap="gray")


@pims.pipeline
def gray(image):
    return image[:, :, 1]  # Take just the green channel


@pims.pipeline
def graysc(image):
    return image


def main():
    # gridpic = gray(pims.open("./particle_tracking/40x_DF_Gridpic.tiff"))
    images = {
        "a1": "./Analyserte/SampleA_1_mag40x_255frames_DF_50msexposure.tif",
        "a2": "./Analyserte/SampleA_2_mag40x_230frames_DF_50msexposure.tif",
        "a3": "./Analyserte/SampleA_3_mag40x_249frames_DF_50msexposure.tif",
        "a4": "./Analyserte/SampleA_4_mag40x_249frames_DF_50msexposure.tif",
        "b1": "./Analyserte/SampleB_1_mag40x_251frames_DF_50msexposure.tif",
        "b2": "./Analyserte/SampleB_2_mag40x_253frames_DF_50msexposure.tif",
        "b3": "./Analyserte/SampleB_3_mag40x_266frames_DF_50msexposure.tif",
    }
    frames = pims.open(images["b2"])

    # f = tp.locate(frames[0], 21, minmass=80, maxsize=2.1, threshold=5)
    f = tp.batch(frames[:], 21, minmass=80, maxsize=2.1, threshold=5)
    t = tp.link(f, 5, memory=3)
    t1 = tp.filter_stubs(t, 25)

    print(t["particle"].nunique())
    print(t1["particle"].nunique())

    # fig, ax = plt.subplots()

    # ax.hist(f["mass"], bins=20)

    tp.annotate(f, frames)

    # plt.imshow(frames[2])
    # plt.show()


if __name__ == "__main__":
    main()
