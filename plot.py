import matplotlib.pyplot as plt

if __name__ == "__main__":
    files = ["result/abc-pdr-ctp-09-07-21-08", "result/abc-pdr-08-31-16-20"]
    for file in files:
        data = []
        with open(file, "r") as file:
            for line in file:
                model, time = line.strip().split()
                if time != "None":
                    print(time)
                    data.append(float(time))
        data = sorted(data)
        plt.plot(range(len(data)), data, marker="x")

    plt.title("result")
    plt.xlabel("solved")
    plt.ylabel("time")
    plt.show()
    plt.savefig("result.png")
