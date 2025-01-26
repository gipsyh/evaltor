import matplotlib.pyplot as plt
from scipy.stats import gmean
from evaluatee import Evaluatee
import math
import numpy as np
import sys


def filter(x: Evaluatee, y: Evaluatee):
    for case in x.cases():
        if y[case] is None:
            print(case, "Timeout")
        else:
            print(case, y[case])


if __name__ == "__main__":
    evaluatee = []
    for file in sys.argv[1:]:
        evaluatee.append(Evaluatee(file))
    filter(evaluatee[0], evaluatee[1])
