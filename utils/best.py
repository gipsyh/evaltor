import sys

if __name__ == "__main__":
    data = {}
    cases = set()
    for file in sys.argv[1:]:
        print(file)
        with open(file, "r") as f:
            for line in f:
                case, time = line.strip().split()
                if case.endswith("aag"):
                    case = case[:-4]
                if case.endswith("aig"):
                    case = case[:-4]
                if case.endswith("btor2"):
                    case = case[:-6]
                case = case.split("/")[-1]
                cases.add(case)

                if time == "Timeout" or time == "Failed":
                    continue
                time = float(time)
                if case in data:
                    data[case].append(time)
                else:
                    data[case] = [time]
    with open("best-result", "w") as result:
        keys = sorted(data.keys())
        for key in keys:
            output = f"{key} {min(data[key])}"
            result.write(output + "\n")
        for c in cases:
            if not c in data:
                output = f"{c} Timeout"
                result.write(output + "\n")
