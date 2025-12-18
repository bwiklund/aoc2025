#!/bin/bash

DAY=$1

gcc day${DAY}.c -o day${DAY}.out && ./day${DAY}.out