
from matplotlib.lines import Line2D
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import os
import pandas as pd
import numpy as np

colors = ['b', 'g', 'r', 'c', 'm']
num_of_densities = 3


def load_benches(path):
    df = pd.read_csv(path, header=None, names=[
                     "size", "dense", "time", "mem_cost"])
    return df


def compare_benches(benches, compare_name):
    plt.tick_params(left=False, right=False, labelleft=False,
                    labelbottom=False, bottom=False)

    fig, ax = plt.subplots(1, len(
        benches), constrained_layout=True, sharex=True, sharey=True, squeeze=False)
    fig.set_size_inches(10, 6)
    fig.text(0.5, -0.02, 'size [num of bits]', ha='center', va='center')
    fig.text(-0.01, 0.5, 'time [ns]', ha='center',
             va='center', rotation='vertical')

    for i, (bench, bench_name) in enumerate(benches):
        for d, (name, group) in enumerate(bench.groupby("dense")):
            ax[0, i].plot(group["size"], group["time"], label=f"density={float(name)*100}%",
                          color=colors[d], marker="o", markersize=3, linewidth=1.0)
        ax[0, i].set_title(bench_name)

    means = np.sort(np.concatenate(
        list(map(lambda x: x[0]["time"].unique(), benches)), axis=0))
    ticks = np.logspace(np.log10(means[0]), np.log10(means[-1]), num=6)
    ticks = list(map(lambda x: round(x, 1), ticks))
    for i in range(len(benches)):
        ax[0, i].set_xscale("log")
        ax[0, i].set_yscale("log")
        ax[0, i].set_yticks(ticks)
        ax[0, i].set_yticklabels(ticks)
        ax[0, i].grid(True)
        ax[0, i].yaxis.set_minor_locator(plt.NullLocator())

    h1, _ = ax[0, 0].get_legend_handles_labels()
    fig.legend(handles=h1, loc='upper center', bbox_to_anchor=(
        0.5, -0.04), fancybox=True, shadow=True, ncol=5)

    plt.draw_all()
    plt.savefig("./plots/{}.svg".format(compare_name),
                format="svg", bbox_inches="tight")
    plt.close(fig)


if __name__ == "__main__":
    benches = []
    for file in os.listdir("./target/results"):
        if file.endswith(".csv"):
            benches.append(
                (load_benches(f"./target/results/{file}"), file[:-4]))
    compare_benches(benches, "benches")
