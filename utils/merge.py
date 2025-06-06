import sys
from evaluatee import Evaluatee


def merge(evaluatee: list[Evaluatee]):
    result = {}
    for case in evaluatee[0].cases():
        result[case] = [f"{e.tag(case)}({e[case]})" for e in evaluatee]
    return result


if __name__ == "__main__":
    evaluatee = []
    for file in sys.argv[1:]:
        evaluatee.append(Evaluatee(file))
    merged = merge(evaluatee)
    with open("merge-result", "w") as result:
        keys = sorted(merged.keys())
        for case in merged:
            output = f"{case} "
            output += " ".join(
                str(time) if time != "None(None)" else "Timeout"
                for time in merged[case]
            )
            result.write(output + "\n")
