#!/bin/bash

echo 0 > /sys/devices/system/cpu/cpu7/online

echo -n "slave_name=test cpu_ids=7 memory=1G@5G fdt=/root/dysche.dtb kernel=/root/kimg" > /sys/dysche/create
