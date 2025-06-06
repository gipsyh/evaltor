import sys
from evaluatee import Evaluatee
import pandas as pd


def merge(evaluatee: list[Evaluatee]):
    result = {}
    for case in evaluatee[0].cases():
        result[case] = [f"{e.tag(case)}({e[case]})" for e in evaluatee]
    return result


if __name__ == "__main__":
    evaluatee = []
    for file in sys.argv[1:]:
        evaluatee.append(Evaluatee(file))  # 假设 Evaluatee(file).name 可获取文件名

    merged = merge(evaluatee)

    rows = {}
    for case, times in merged.items():
        processed = [time if time != "None(None)" else "Timeout" for time in times]
        rows[case] = processed

    df = pd.DataFrame.from_dict(rows, orient="index")

    df.columns = [e.name for e in evaluatee]

    df.to_csv("merge-result.csv", index_label="Case")
