#!/bin/bash

echo 0 > /sys/devices/system/cpu/cpu3/online

echo -n "name=test cpu_ids=3 memory=1G@5G fdt=/root/dysche.dtb kernel=/root/kimg" > /sys/dysche/create
