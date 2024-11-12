#!/bin/sh

I=0
for LINK in $(head -n 10 ./dataset.csv)
do
    wget $LINK -O "$I.gif"
    I=$((I+1))
done
