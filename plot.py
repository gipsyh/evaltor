import matplotlib.pyplot as plt
import sys

if __name__ == "__main__":
    for file in sys.argv[1:]:
        data = []
        with open(file, "r") as file:
            for line in file:
                model, time = line.strip().split()
                if time != "Timeout" and time != "Failed":
                    print(time)
                    data.append(float(time))
        data = sorted(data)
        plt.plot(range(len(data)), data, marker="x")

    plt.title("result")
    plt.xlabel("solved")
    plt.ylabel("time")
    plt.show()
    plt.savefig("result.png")
