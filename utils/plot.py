import matplotlib.pyplot as plt
from evaluatee import Evaluatee


def sub_plot(fg, e: Evaluatee, line=None):
    # def sub_plot(fg, res, labels, colors, linestyles):
    data = list(e.data.values())
    data = sorted(data)
    X = []
    Y = []
    for t in range(0, e.TIMEOUT + 1, 30):
        X.append(t)
        Y.append(sum(1 for d in data if d <= t))

    fg.plot(
        X,
        Y,
        label=e.name,
        linestyle=line,
        linewidth=1.3,
        markerfacecolor="none",
    )


def plot(evaluatee: list[Evaluatee]):
    fig, (ax1) = plt.subplots(1, 1, figsize=(8, 3.8))

    line = ["-", "-.", ":", "--"]
    if len(line) < len(evaluatee):
        line.extend([None] * (len(evaluatee) - len(line)))
    for e, l in zip(evaluatee, line):
        sub_plot(
            ax1,
            e,
            l,
        )
    max_sol = max(e.num_success() for e in evaluatee)
    min_sol = min(e.num_success() for e in evaluatee)
    ax1.set_xlabel("Time(s)")
    ax1.set_ylabel("Cases Solved")
    ax1.legend(bbox_to_anchor=(1.05, 0.5), loc="center left", frameon=False)
    ax1.set_ylim(min_sol * 0.5, max_sol + 20)

    # plt.subplots_adjust(wspace=0)
    plt.subplots_adjust(right=0.7)
    plt.show()
    plt.savefig("plot.png", dpi=1000)
