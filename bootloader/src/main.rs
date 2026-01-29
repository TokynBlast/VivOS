#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::{print, println};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, FileAttribute, FileMode};

#[entry]
fn main() -> Status {
  uefi::helpers::init(&mut system_table).unwrap();
  //uefi::helpers::init().unwrap();

  println!("Booting VivOS"); // :3

  struct BootInfo {
    secure_boot: bool,
    // buffer_loc: u64=

  }

  let bt = &uefi::helpers::boot_services().unwrap();

  let info = BootInfo {
    secure_boot: false
    // bufffer_loc: 2097154 // book 2
  };

  loop {}
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  println!("FATAL ERROR, TRY AGAIN!\n{}", _info);
  loop {}
}
