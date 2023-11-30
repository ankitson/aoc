#!/bin/bash
set -e 

day=$1

if [ -z "$day" ] || ! [[ "$day" =~ ^[0-9]+$ ]]; then
    echo "Error: Day is empty or not a valid number"
    exit 1
fi

if [ ${#day} = 1 ]; then
    day=0$day;
fi

if [ -d "day$day" ]; then
    echo "Error: Directory day$day already exists"
    exit 1
fi

echo "Generating template for day $day";

cp -R template day$day
fdfind -t file --glob "*" -0 day$day/ | xargs --null -I{} sed -i "s/{DAY_NUM}/$day/g" {}
touch inputs/day$day.txt
touch inputs/sample$day.txt