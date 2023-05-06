#!/bin/bash

for i in {1..32}
do
    while [[ $(squeue -u jht517 | wc -l) > 2 ]]
    do
	squeue -u jht517
        sleep 5
    done
    sbatch ./$i.job
done
