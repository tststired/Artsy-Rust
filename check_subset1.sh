#!/bin/bash

folder_path="logo_examples"
file_list=""

# Use a for loop to iterate through the files in the folder
for file in "$folder_path"/1_0*; do
    # Check if the file starts with "1_"
    echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
    echo $file
    ./check.sh "$file"
    echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
done

echo "$file_list"
