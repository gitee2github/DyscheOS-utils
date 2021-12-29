yum install bash --installroot /root/root-t

sudo mknod /root/root-t/dev/console c 5 1
sudo chown root:tty /root/root-t/dev/console

find . | cpio --quiet -H newc -o | gzip -9 -n > ../initramfs.img
