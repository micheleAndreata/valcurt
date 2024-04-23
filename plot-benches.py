
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


def is_pareto_efficient(costs):
    """
    Find the pareto-efficient points
    :param costs: An (n_points, n_costs) array
    :return: A (n_points, ) boolean array, indicating whether each point is Pareto efficient
    """
    is_efficient = np.ones(costs.shape[0], dtype=bool)
    for i, c in enumerate(costs):
        is_efficient[i] = np.all(np.any(costs[:i] <= c, axis=1)) and np.all(
            np.any(costs[i+1:] <= c, axis=1))
    return is_efficient


def draw_pareto_front(benches, compare_name):
    fig, ax = plt.subplots(1, 1, constrained_layout=True)
    ax.set_ylabel("memory cost [%]")
    ax.set_xlabel("time [ns]")

    for i, (bench, bench_name) in enumerate(benches):
        bench = bench[bench["dense"] == 0.5]
        pareto = bench[is_pareto_efficient(bench[["time", "mem_cost"]].values)]
        ax.plot(pareto["time"], pareto["mem_cost"], label=bench_name,
                color=colors[i], marker="o", markersize=3, linewidth=1.0)
    ax.grid(True)

    h1, _ = ax.get_legend_handles_labels()
    fig.legend(handles=h1, loc='upper center', bbox_to_anchor=(
        0.5, -0.04), fancybox=True, shadow=True, ncol=5)

    plt.draw_all()
    plt.savefig("./plots/{}.svg".format(compare_name),
                format="svg", bbox_inches="tight")
    plt.close(fig)


def scatter(benches, plot_name):
    fig, ax = plt.subplots(1, 1, constrained_layout=True)
    ax.set_ylabel("memory cost [%]")
    ax.set_xlabel("time [ns]")

    for i, (bench, bench_name) in enumerate(benches):
        bench = bench[bench["dense"] == 0.5]
        bench = bench[bench["size"] == 1073741826]
        ax.plot(bench["time"], bench["mem_cost"], label=bench_name,
                color=colors[i], marker="o", markersize=6, linewidth=1.0)
    ax.grid(True)

    h1, _ = ax.get_legend_handles_labels()
    fig.legend(handles=h1, loc='upper center', bbox_to_anchor=(
        0.5, -0.04), fancybox=True, shadow=True, ncol=5)

    plt.draw_all()
    plt.savefig("./plots/{}.svg".format(plot_name),
                format="svg", bbox_inches="tight")
    plt.close(fig)


if __name__ == "__main__":
    benches = []
    for file in os.listdir("./target/results"):
        if file.endswith(".csv"):
            benches.append(
                (load_benches(f"./target/results/{file}"), file[:-4]))
    compare_benches(benches, "benches")
    draw_pareto_front(benches, "pareto")
    scatter(benches, "scatter")
