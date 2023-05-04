#!/bin/bash

for i in {1..32}
do
    while [[ $(squeue -u jht517 | wc -l) > 2 ]]
    do
        sleep 0.05
    done
    ./$i.job
done
