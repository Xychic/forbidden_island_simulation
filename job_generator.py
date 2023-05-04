

for i in range(32):
    job = i+1
    open(f"{job}.job", "w").write(f"""#!/bin/bash
#SBATCH --job-name=FIS_{job}
#SBATCH --time=00:10:00                 # Maximum time (HH:MM:SS)
#SBATCH --ntasks=32                      # run on a single CPU
#SBATCH --mem=100mb                       # reserve 1GB memory for job
#SBATCH --output=./logs/{job}.log                 # standard output and error log
#SBATCH --partition=teach               # run in the teaching queue
#SBATCH --nodes=1

echo {job}
./forbidden_island_simulation -g 5000000 -t {job}
""")