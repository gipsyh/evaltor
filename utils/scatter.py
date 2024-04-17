import matplotlib.pyplot as plt
import sys
from scipy.stats import gmean

timeout = 1000

def parse_time(time):
    if time == "Timeout":
        return timeout
    elif time == "Failed":
        return timeout
    else:
        time = float(time)
        if time < 0.01:
            time = 0.01
        return time


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
    num_x = 0
    num_y = 0
    keys = sorted(data.keys())
    speedup = []
    for key in keys:
        if len(data[key]) < 2:
            continue
        bsdp = data[key][0] == "Timeout" or data[key][1] == "Timeout"
        x = parse_time(data[key][0])
        y = parse_time(data[key][1])        
        X.append(x)
        Y.append(y)

        if x <= 1 and y <= 1:
            continue
        if not bsdp:
            speedup.append(y / x)
        if x < y:
            num_x += 1
        elif x > y:
            num_y += 1
        

    print((num_x, num_y))
    gmean = gmean(speedup)
    print(gmean)
    plt.axis('equal')
    plt.scatter(X, Y, marker='x')
    plt.xscale('log')
    plt.yscale('log')
    plt.xlabel(name[0])
    plt.ylabel(name[1])
    plt.xlim(0.1, timeout + 500)
    plt.ylim(0.1, timeout + 500)
    plt.plot([0, timeout], [0, timeout], linestyle='dashed', color='grey')
    plt.plot([0, timeout], [0, timeout * gmean], linestyle='dashed', color="#721454")
    plt.show()
    fig = plt.gcf()
    fig.set_size_inches(5, 5)
    plt.savefig("scatter.png", dpi=500)
