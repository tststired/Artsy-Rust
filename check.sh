#!/bin/bash

#usage "pathname, x, y", if blank defaults used

mine="cargo run"
refer="6991 rslogo"
path="$1"
x="$2"
y="$3"
size="$x $y"
output1="output_mine.svg"
output2="output_ref.svg"

if [ -z "$path" ]; then
    path="logo_examples/1_00_penup_pendown.lg"
fi

if [ -z "$y" ] || [ -z "$x" ]; then
    size="200 200"
fi

$mine $path $output1 $size 
$refer $path $output2 $size 

diff $output1 $output2
