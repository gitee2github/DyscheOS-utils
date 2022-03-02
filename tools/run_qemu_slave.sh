#!/bin/sh

qemu-system-riscv64 -nographic \
		    -bios fw_dynamic.elf \
		    -machine virt \
		    -smp 2 \
		    -m 1G \
		    -kernel kimg-s \
    		    -append 'rdinit=/bin/sh console=ttyS0 earlycon'

#		    -append 'rdinit=/bin/sh root=/dev/vda1 irqpoll mem=1G rootwait rw console=ttyS0 systemd.default_timeout_start_sec=60 selinux=0 highres=off earlycon'
#    		    -append 'rdinit=/bin/sh irqpoll console=sbi earlycon=sbi'
# dysche_reserve=6G@0x100000000
#		    -initrd initramfs.img \
