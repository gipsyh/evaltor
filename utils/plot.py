import matplotlib.pyplot as plt
import sys

TIMEOUT = 1000

if __name__ == "__main__":
    for file_name in sys.argv[1:]:
        data = []
        par2 = 0
        cases = 0
        with open(file_name, "r") as file:
            for line in file:
                model, time = line.strip().split()
                cases += 1
                if time != "Timeout" and time != "Failed":
                    t = float(time)
                    data.append(float(time))
                    par2 += t
                else:
                    par2 += TIMEOUT * 2
        data = sorted(data)
        print(file_name, len(data), '{:.2f}'.format(par2 / cases))
        plt.plot(range(len(data)), data, marker="x", label=file_name)

    plt.title("result")
    plt.xlabel("solved")
    plt.ylabel("time")
    plt.legend()
    plt.show()
    plt.savefig("result.png", dpi=500)
