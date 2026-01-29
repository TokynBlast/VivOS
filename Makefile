.PHONY: all build run clean

all: build

build:
	@echo "🦀 Building VivOS UEFI Bootloader..."
	cargo +nightly build --release --target x86_64-unknown-uefi -Z build-std=core,compiler_builtins
	@echo "📦 Creating disk image..."
	@if [ ! -f boot.img ]; then \
		dd if=/dev/zero of=boot.img bs=1M count=64; \
		mkfs.vfat boot.img; \
	fi
	@echo "🗻 Making mnt folder..."
	@mkdir -p mnt
	@sudo mount boot.img mnt
	@sudo mkdir -p mnt/EFI/BOOT
	@sudo cp target/x86_64-unknown-uefi/release/bootloader.efi mnt/EFI/BOOT/BOOTX64.EFI
	@sudo umount mnt
	@rmdir mnt
	@echo "✅ Build complete!"

run: build
	@echo "🚀 Running VivOS in QEMU..."
	qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive file=boot.img,format=raw

clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -f boot.img
	rm -rf mnt
	@echo "✅ Clean complete!"
