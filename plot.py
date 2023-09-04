import matplotlib.pyplot as plt

if __name__ == "__main__":
    data = []
    with open("result/abc-pdr-09-01-13-00", "r") as file:
        for line in file:
            model, time = line.strip().split()
            if time != "None":
                print(time)
                data.append(float(time))
    data = sorted(data)
    print(data)
    print(len(data))

    plt.plot(range(len(data)), data, marker="o")
    plt.title("result")
    plt.xlabel("solved")
    plt.ylabel("time")
    plt.show()
    plt.savefig("result.png")
