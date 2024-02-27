#!/bin/sh

for X in `echo "$1" | sed -r 's/^0x//;s/.{8}/& /g'`; do 
    printf "%d " "0x$X"
done
printf '\n'
