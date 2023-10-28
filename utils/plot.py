import matplotlib.pyplot as plt
import sys

if __name__ == "__main__":
    for file_name in sys.argv[1:]:
        data = []
        with open(file_name, "r") as file:
            for line in file:
                model, time = line.strip().split()
                if time != "Timeout" and time != "Failed":
                    data.append(float(time))
        data = sorted(data)
        plt.plot(range(len(data)), data, marker="x", label = file_name)

    plt.title("result")
    plt.xlabel("solved")
    plt.ylabel("time")
    plt.legend()
    plt.show()
    plt.savefig("result.png", dpi = 1000)
