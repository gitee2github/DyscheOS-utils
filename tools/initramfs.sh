yum install bash --installroot /root/root-t
find . | cpio --quiet -H newc -o | gzip -9 -n > ../initramfs.img
