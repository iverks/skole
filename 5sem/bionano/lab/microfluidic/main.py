import pandas as pd
import numpy as np
from pprint import pprint
import matplotlib.pyplot as plt
from scipy import stats
from scipy.optimize import curve_fit
from scipy.signal import lfilter
from scipy.special import erf


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
        if file in [2, 3, 4]:
            xvals = np.max(xvals) - xvals

        # Find middle to let it be x = 0
        # maxy = np.max(yvals)
        # miny = np.min(yvals)
        # midy = miny + ((maxy - miny) / 2)
        # middle = yvals[(np.abs(yvals - midy)).argmin()]
        # middle = xvals[yvals == middle]
        middle = 0
        if file == 1:
            middle = 235
        elif file == 2:
            middle = 220
        elif file == 3:
            middle = 227
        elif file == 4:
            middle = 230

        xvals -= middle

        # distance = 768  # um
        # flow_rate = 20  # ul/s
        # cross_area = 500 * 75  # um * um
        # t = distance * cross_area / flow_rate
        # c = df["Gray_Value"][40]

        cmax = 200
        cmin = 0
        if file == 1:
            cmax = 217
            cmin = 190
        elif file == 2:
            cmax = 230
            cmin = 188
        elif file == 3:
            cmax = 212
            cmin = 166
        elif file == 4:
            cmax = 200
            cmin = 157

        c = cmax - cmin
        D = 5
        t = 1  # ??????

        yvals -= cmin

        def fittefunksjon(x, D):
            return c / 2 - c / 2 * erf(x / np.sqrt(4 * D * t))

        params, _ = curve_fit(f=fittefunksjon, xdata=xvals, ydata=yvals)
        D = params[0]
        results[file] = {
            "xvals": xvals,
            "yvals": yvals,
            "D": D,
            "funcres": fittefunksjon(xvals, D),
        }

        # plt.plot(xvals, yvals)
        # plt.plot(xvals, fittefunksjon(xvals, D))
        # plt.show()
    _, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2)
    axs = [ax1, ax2, ax3, ax4]
    for file in filestorun:
        xvals = results[file]["xvals"]
        yvals = results[file]["yvals"]
        axs[file - 1].plot(xvals, yvals)
        axs[file - 1].plot(results[file]["xvals"], results[file]["funcres"])
        axs[file - 1].set_title(str(file))
    plt.show()
    pprint({file: res["D"] for file, res in results.items()})


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


if __name__ == "__main__":
    main()

    # test()
