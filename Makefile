# Tools
CC = gcc
LD = ld
QEMU = qemu-system-x86_64

# Flags
CFLAGS = -ffreestanding -fno-stack-protector -mno-red-zone -Wall -Wextra -Iinclude -Iinclude/limine-protocol/include
LDFLAGS = -T linker/x86_64.lds -nostdlib -static -no-pie -m elf_x86_64 -z max-page-size=0x1000

# Files
TARGET = kernel

SRC = \
	src/main.c \
	src/debug.c \
	src/string.c \
	src/arch/x64/serial.c

OBJ = $(SRC:src/%.c=build/%.o)

ISO = kernel.iso
ISO_ROOT = build/iso

# Build kernel
build: $(TARGET)

$(TARGET): $(OBJ)
	$(LD) $(LDFLAGS) $(OBJ) -o $@

build/%.o: src/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# Create ISO
iso: $(TARGET)
	@rm -rf $(ISO_ROOT)
	@mkdir -p $(ISO_ROOT)/boot

	cp $(TARGET) $(ISO_ROOT)/boot/kernel
	cp limine.conf $(ISO_ROOT)/
	cp limine/limine-bios.sys $(ISO_ROOT)/boot/
	cp limine/limine-bios-cd.bin $(ISO_ROOT)/boot/
	cp limine/limine-uefi-cd.bin $(ISO_ROOT)/boot/

	xorriso -as mkisofs \
		-b boot/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO)

	limine/limine bios-install $(ISO)

# Clean build files
clean:
	rm -rf build $(TARGET) $(ISO)

# Run kernel
run: iso
	$(QEMU) -cdrom $(ISO) -serial stdio

# Format file with clang-format
format:
	find src include -type f \( -name '*.c' -o -name '*.h' \) \
		-exec clang-format -i {} +

.PHONY: build iso run clean format
