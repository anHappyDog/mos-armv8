#!/bin/bash
QEMU="${USING_QEMU:-qemu-system-aarch64}"
DEVICE="${DEVICE:-raspi4b}"
CPU="${CPU:-cortex-a72}"
SMP="${QEMU_SMP:-4}"
MEM="${QEMU_MEM:-2G}"
IMG_FILE="${QEMU_IMG:-fs.img}"
DTB_FILE="${DTB:-raspi4b.dtb}"
DTS_FILE=${DTS:-raspi4b.dts}
KERNEL="${1}"


MODE="${QEMU_MODE:-run}"

if [ "$MODE" == "debug" ]; then
    exec "$QEMU" \
        -smp "$SMP" \
        -cpu "$CPU" \
        -m "$MEM" \
        -nographic \
        -M "$DEVICE" \
        -dtb "$DTB_FILE" \
        -kernel "$KERNEL" \
        -gdb tcp::1234 -S \
        -no-reboot
elif [ "$MODE" == "dumpdtb" ]; then
    "$QEMU" \
        -smp "$SMP" \
                -cpu "$CPU" \
        -m "$MEM" \
        -nographic \
        -M "$DEVICE",dumpdtb="$DTB_FILE"
    dtc -I dtb -O dts -o "$DTS_FILE" "$DTB_FILE"

elif [ "$MODE" == "run" ]; then
    exec "$QEMU" \
        -smp "$SMP" \
                -cpu "$CPU" \
        -m "$MEM" \
        -nographic \
        -M "$DEVICE" \
        -dtb "$DTB_FILE" \
        -kernel "$KERNEL" \
        -no-reboot
else
    echo "QEMU_MODE env illegal."
fi
