import matplotlib.pyplot as plt
import numpy as np
import sys

def parse_time(time):
    if time == "Timeout":
        return 1000
    else:
        return float(time)

if __name__ == "__main__":
    data = {}
    name = []
    for file in sys.argv[1:3]:
        name.append(file)
        with open(file, 'r') as f:
            for line in f:
                case, time = line.strip().split()
                if case.endswith("aag"):
                    case = case[:-3] + "aig"
                if case in data:
                    data[case].append(time)
                else:
                    data[case] = [time]

    X = []
    Y = []
    keys = sorted(data.keys())
    for key in keys:
        X.append(parse_time(data[key][0]))
        Y.append(parse_time(data[key][1]))
    
    plt.scatter(X, Y, marker='x')
    plt.xscale('log')
    plt.yscale('log')
    plt.xlabel(name[0])
    plt.ylabel(name[1])
    plt.plot([0, 1000], [0, 1000], linestyle='dashed', color='grey')
    plt.show()
