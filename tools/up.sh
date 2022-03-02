#!/bin/sh

CWD=`pwd`

echo "--- build kernel ---"
KSRC="/home/ll/riscv/openEuler/kernel"
KBLD="/home/ll/riscv/openEuler/kernel-build"
KIMG="$KBLD/arch/riscv/boot/Image"
# rebuild kernle
cd $KSRC
make ARCH=riscv O=$KBLD CROSS_COMPILE=riscv64-linux-gnu-
cd $CWD
/bin/cp -f $KIMG kimg

echo "--- build openSBI binary ---"
SBI_SRC="/home/ll/repo/opensbi"
SBI_OUT="$SBI_SRC/ll-out"
SBI_BIN="$SBI_OUT/platform/generic/firmware/fw_dynamic.elf"
cd $SBI_SRC
rm -rf $SBI_OUT
make PLATFORM=generic CROSS_COMPILE=riscv64-linux-gnu- FW_PAYLOAD=y O=$SBI_OUT FW_PAYLOAD_PATH=$KIMG FW_DYNAMIC=y
cd $CWD
cp $SBI_BIN .

echo "--- run qemu ---"
#./r.sh

echo "--- Done ---"
