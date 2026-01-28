#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Clear screen
    system_table.stdout().clear().unwrap();
    system_table.stdout()
        .output_string(cstr16!("🦀 Rust UEFI Bootloader :3\n"))
        .unwrap();

    loop {}

    status::SUCCESS  // unreachable but needed for type
}
