#!/bin/bash
input_id=$1
cookies=$AOC_SESSION

if [ -z "$1" ]
  then
    echo "Need an input (number)!"
    exit -1
fi

if [ -z "$cookies" ]
  then
    echo "AOC_SESSION not set."
    exit -1
fi

mkdir -p inputs
curl --cookie "$cookies"  https://adventofcode.com/2024/day/$input_id/input -o ./inputs/$input_id
