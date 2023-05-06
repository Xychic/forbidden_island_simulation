

for i in range(32):
    j = i+1
    # print(open(f"{j}.log").read().split("\n\n")[0].split("\n")[-1].split(" ")[1].replace(",",""))
    data = [int(l.split()[1].replace(",","")) for l in open(f"{j}.log").read().split("\n\n")[0].split("\n")[1:]]
    min_ = min(data)
    max_ = max(data)
    avg = data[-1]
    # input(f"avg: {avg} min: {min_} max: {max_}")
    print(f"{j} & {avg} \\\\\n\\hline")