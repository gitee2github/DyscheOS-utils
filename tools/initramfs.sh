#!/bin/bash

ROOTFS_ROOT="/root/root-t/"
yum install bash --installroot $ROOTFS_ROOT

sudo mknod $ROOTFS_ROOT/dev/console c 5 1
sudo chown root:tty $ROOTFS_ROOT/dev/console

cd $ROOTFS_ROOT
sudo rm -rf usr/lib/locale/locale-archive*
sudo rm -rf var/cache/dnf/
sudo rm -rf usr/share/locale/

sudo find . | sudo cpio --quiet -H newc -o  > ../initramfs.cpio
cd ..


