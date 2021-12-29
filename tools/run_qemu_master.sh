#!/bin/sh

qemu-system-riscv64 -nographic \
		    -bios fw_dynamic.elf \
		    -machine virt \
		    -smp 8 \
		    -m 8G \
		    -kernel kimg-m \
		    -drive file=oe-rv-rv64g-30G.qcow2,format=qcow2,id=hd0 \
		    -object rng-random,filename=/dev/urandom,id=rng0 \
		    -device virtio-rng-device,rng=rng0 \
		    -device virtio-blk-device,drive=hd0 \
		    -device virtio-net-device,netdev=usernet \
		    -netdev user,id=usernet,hostfwd=tcp::12055-:22 \
		    -append 'rdinit=/bin/sh root=/dev/vda1 irqpoll mem=1G rootwait rw console=ttyS0 systemd.default_timeout_start_sec=60 selinux=0 highres=off earlycon' \
		    -serial tcp::12121,server,nowait \
                    -serial tcp::12122,server,nowait \
                    -serial tcp::12123,server,nowait \
		    -serial tcp::12124,server,nowait

