#!/bin/bash
DAY=$1
sed "s/dayX/day${DAY}/g" src/template.txt > src/day${DAY}.rs
sed -i '' "s|// NEXT|pub mod day${DAY};\n// NEXT|" src/main.rs
