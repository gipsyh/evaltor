from evaluatee import Evaluatee


def analyze(evaluatee: list[Evaluatee]):
    unique = {e: [] for e in evaluatee}
    best = {e: [] for e in evaluatee}
    for case in evaluatee[0].cases():
        res = [e[case] for e in evaluatee]
        if res.count(None) == len(res) - 1:
            for e in evaluatee:
                if e[case] is not None:
                    unique[e].append(case)
        
        res = [x for x in res if x is not None]
        if not res:
            continue
        
        min_res = min(res)
        min_count = res.count(min_res)
        
        if min_count == 1:
            for e in evaluatee:
                if e[case] == min_res:
                    best[e].append(case)
    for e in evaluatee:
        print(e.name, len(unique[e]), len(best[e]))
