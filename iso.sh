#!/bin/sh
set -e
. ./build.sh

mkdir -p build
mkdir -p build/boot
mkdir -p build/boot/grub

cp sysroot/boot/pOSt.kernel build/boot/pOSt.kernel
cat > build/boot/grub/grub.cfg << EOF
menuentry "pOSt" {
	multiboot /boot/pOSt.kernel
}
EOF
grub2-mkrescue -o pOSt.iso build
