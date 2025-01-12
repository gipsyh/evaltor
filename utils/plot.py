import matplotlib.pyplot as plt
from evaluatee import Evaluatee


def sub_plot(fg, e: Evaluatee):
    # def sub_plot(fg, res, labels, colors, linestyles):
    data = list(e.data.values())
    data = sorted(data)
    label = e.name
    X = []
    Y = []
    for t in range(0, e.TIMEOUT + 1, 30):
        X.append(t)
        Y.append(sum(1 for d in data if d <= t))
    fg.plot(
        X,
        Y,
        label=label,
        # color=color,
        # linestyle=linestyle,
        linewidth=1.3,
        markerfacecolor="none",
    )


def plot(evaluatee: list[Evaluatee]):
    fig, (ax1) = plt.subplots(1, 1, figsize=(6, 3.8))

    # ["green", "#2E77B2", "orange", "red"],
    # ["-.", ":", "--", "-"],
    for e in evaluatee:
        sub_plot(
            ax1,
            e,
        )
    max_sol = max(e.num_success() for e in evaluatee)
    min_sol = min(e.num_success() for e in evaluatee)
    ax1.set_xlabel("Time(s)")
    ax1.set_ylabel("Cases Solved")
    ax1.legend()
    ax1.set_ylim(min_sol * 0.5, max_sol + 10)

    plt.subplots_adjust(wspace=0)
    plt.show()
    plt.savefig("plot.png", dpi=1000)
