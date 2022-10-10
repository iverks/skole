import pandas as pd
import numpy as np
from pprint import pprint
import matplotlib.pyplot as plt
from scipy import stats
from scipy.optimize import curve_fit
from scipy.signal import lfilter
from scipy.special import erf


def fittefunksjon(x, D_t, c_max, c_min, x_0):
    c = c_min / 2 - c_min / 2 * erf((x - x_0) / np.sqrt(4 * D_t))
    return 10 ** (c_max - c)


def main():
    files = {
        1: "./resultater/bilde_3_strek_1_768um.csv",
        2: "./resultater/bilde_3_strek_2_1297um.csv",
        3: "./resultater/bilde_3_strek_3_1931um.csv",
        4: "./resultater/bilde_3_strek_4_2593um.csv",
    }
    results = {}
    filestorun = [1, 2, 3, 4]

    for file in filestorun:
        filename = files[file]
        df = pd.read_csv(filename)
        # Remove outlier data
        # df = df[(np.abs(stats.zscore(df["Gray_Value"])) < 1.5)] # nvm dont do it

        xvals = np.array(df["Distance_(um)"])
        yvals = np.array(df["Gray_Value"])
        idxs = xvals < 481
        xvals = xvals[idxs]
        yvals = yvals[idxs]

        # # Reverse order of x and y values
        if file in [1]:
            xvals = np.max(xvals) - xvals

        params, _ = curve_fit(f=fittefunksjon, xdata=xvals, ydata=yvals)
        results[file] = {"xvals": xvals, "yvals": yvals, "params": params}
        pprint(params)

    _, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2)
    axs = [ax1, ax2, ax3, ax4]
    for file in filestorun:
        xvals = results[file]["xvals"]
        yvals = results[file]["yvals"]
        params = results[file]["params"]
        D_t, c_max, c_min, x_0 = params
        axs[file - 1].plot(xvals, yvals)
        axs[file - 1].plot(xvals, fittefunksjon(xvals, D_t, c_max, c_min, x_0))
        axs[file - 1].set_title(str(file))
    plt.show()


def test():
    files = {
        1: "./resultater/bilde_3_strek_1_768um.csv",
        2: "./resultater/bilde_3_strek_2_1297um.csv",
        3: "./resultater/bilde_3_strek_3_1931um.csv",
        4: "./resultater/bilde_3_strek_4_2593um.csv",
    }
    results = {}
    filestorun = [1]
    for file in filestorun:
        filename = files[file]
        df = pd.read_csv(filename)
        # Remove outlier data
        # df = df[(np.abs(stats.zscore(df["Gray_Value"])) < 1.5)] # nvm dont do it

        xvals = np.array(df["Distance_(um)"])
        yvals = np.array(df["Gray_Value"])

        # # Reverse order of x and y values
        if file in [2, 3, 4]:
            xvals = np.max(xvals) - xvals

        results[file] = {
            "xvals": xvals,
            "yvals": yvals,
        }
    _, ax = plt.subplots()
    for file in filestorun:
        xvals = results[file]["xvals"]
        yvals = results[file]["yvals"]
        ax.plot(xvals, yvals)
    plt.show()
    pprint({file: res["D"] for file, res in results.items()})


def test2():
    x = np.linspace(0, 400, 100)
    plt.plot(fittefunksjon(x, 1e-4, 220, 190, 205))
    plt.show()


if __name__ == "__main__":
    main()

    test2()
