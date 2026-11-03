#!/bin/sh
set -e
cargo build

rm -rf iso && mkdir -p iso/boot/limine iso/EFI/BOOT
cp target/x86_64-unknown-none/debug/zen-kernel iso/boot/
cp limine.conf iso/boot/limine/
cp Limine/limine-bios.sys Limine/limine-bios-cd.bin Limine/limine-uefi-cd.bin iso/boot/limine/
cp Limine/BOOTX64.EFI iso/EFI/BOOT/

xorriso -as mkisofs -R -r -J \
  -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table \
  -hfsplus -apm-block-size 2048 \
  --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image \
  --protective-msdos-label iso -o zen.iso

./Limine/limine bios-install zen.iso

qemu-system-x86_64 -cdrom zen.iso -debugcon stdio -no-reboot -d int,cpu_reset