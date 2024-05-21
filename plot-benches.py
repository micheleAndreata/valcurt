from matplotlib.lines import Line2D
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import os
import json
import math
import pandas as pd
import numpy as np

colors = ['b', 'g', 'r', 'c', 'm', 'purple',
          'gold', 'teal', 'orange', 'brown', 'pink']
markers = np.array(["v", "o", "+", "*", "^", "s", "D", "x"])


def load_benches(base_path):
    benches_list = []

    for dir in sorted([d for d in os.listdir(base_path) if os.path.isdir(base_path + d)]):
        run_name = dir.split("_")
        data = {}

        data["size"] = int(run_name[0], 10)
        data["dense"] = float(run_name[1])
        data["rep"] = int(run_name[2], 10)

        path = base_path + dir + "/new/estimates.json"
        with open(path, "r") as f:
            estimates = json.load(f)
            data["time"] = estimates["mean"]["point_estimate"]
        benches_list.append(data)

    benches_df = pd.DataFrame(benches_list)
    benches_df = benches_df.groupby(
        ["size", "dense"], as_index=False)["time"].mean()

    mem_cost_df = pd.read_csv(
        base_path + "mem_cost.csv", header=None, names=["size", "dense", "mem_cost"])
    benches_df = pd.merge(benches_df, mem_cost_df,
                          how="left", on=["size", "dense"])

    benches_df = benches_df.sort_values(by="size", ignore_index=True)
    return benches_df


def compare_benches(benches, compare_name, op_type):
    fig, ax = plt.subplots(1, 3, constrained_layout=True,
                           sharex=True, sharey=True, squeeze=False)
    fig.set_size_inches(10, 6)
    fig.text(0.5, -0.02, 'size [num of bits]', ha='center', va='center')
    fig.text(-0.01, 0.5, f'time [ns/{op_type}]', ha='center',
             va='center', rotation='vertical')

    for i, (bench, bench_name) in enumerate(benches):
        for d, (name, group) in enumerate(bench.groupby("dense")):
            ax[0, d].plot(group["size"], group["time"], label=bench_name,
                          color=colors[i], marker=markers[i], markersize=3, linewidth=1.0)
            ax[0, d].set_title(f"density={float(name)*100}%")
            ax[0, d].grid(True)
            ax[0, d].set_xscale("log")
            ax[0, d].set_yscale("log")

    times = np.sort(np.concatenate(
        list(map(lambda x: x[0]["time"].unique(), benches)), axis=0))
    ticks = np.logspace(np.log10(times[0]), np.log10(times[-1]), num=8)
    ticks = list(map(lambda x: math.ceil(x), ticks))
    ax[0, 0].set_yticks(ticks)
    ax[0, 0].set_yticklabels(ticks)
    ax[0, 0].yaxis.set_minor_locator(plt.NullLocator())

    h1, _ = ax[0, 0].get_legend_handles_labels()
    fig.legend(handles=h1, loc='upper center', bbox_to_anchor=(
        0.5, -0.04), fancybox=True, shadow=True, ncol=3)

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
        is_efficient[i] = np.all(np.any(costs[:i] > c, axis=1)) and np.all(
            np.any(costs[i+1:] > c, axis=1))
    return is_efficient


def draw_pareto_front(benches, compare_name):
    fig, ax = plt.subplots(1, 1, constrained_layout=True)
    fig.set_size_inches(10, 6)
    ax.set_ylabel("memory cost [%]")
    ax.set_xlabel("time [ns]")

    bench_per_len = []
    lens = benches[0][0]["size"].unique()
    for l in lens:
        bench_per_len.append([])
        for bench, _ in benches:
            b = bench[bench["dense"] == 0.5]
            b = b[b["size"] == l]
            bench_per_len[-1].append(np.ndarray.flatten(
                b[["time", "mem_cost"]].values))

    for i, bench in enumerate(bench_per_len):
        bench = np.array(bench)
        pareto = bench[is_pareto_efficient(bench)]
        pareto = pareto[np.argsort(pareto[:, 0])]
        ax.plot(pareto[:, 0], pareto[:, 1], label=f"size={lens[i]}",
                color=colors[i], linewidth=1.0)
        for j, p in enumerate(bench):
            if p in pareto:
                plt.scatter(p[0], p[1], color=colors[i],
                            marker=markers[j], s=20)
    ax.grid(True)
    handles = []

    for i, l in enumerate(lens):
        handles.append(mpatches.Patch(
            color=colors[i], label=f"size=2^{math.floor(math.log2(l))}"))

    for i, bench in enumerate(benches):
        handles.append(
            Line2D([0], [0], color='black', marker=markers[i], markersize=5, label=bench[1]))

    fig.legend(handles=handles, loc='upper center', bbox_to_anchor=(
        0.5, -0.04), fancybox=True, shadow=True, ncol=5)

    plt.draw_all()
    plt.savefig("./plots/{}.svg".format(compare_name),
                format="svg", bbox_inches="tight")
    plt.close(fig)


if __name__ == "__main__":
    benches = []
    for file in os.listdir("target/criterion/"):
        if os.path.isdir(f"target/criterion/{file}/"):
            benches.append(
                (load_benches(f"target/criterion/{file}/"), file))
    compare_benches(benches, "benches", "select")
    draw_pareto_front(benches, "pareto")
