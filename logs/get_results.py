

for i in range(32):
    j = i+1
    print(open(f"{j}.log").read().split("\n\n")[0].split("\n")[-1].split(" ")[1].replace(",",""))