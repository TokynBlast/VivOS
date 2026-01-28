#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::{print, println};

#[entry]
fn main() -> Status {
  uefi::helpers::init().unwrap();

  println!("Booting VivOS"); // :3

  struct BootInfo {
    secure_boot: false
  }

  loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  println!("FATAL ERROR, TRY AGAIN!\n{}", _info);
  loop {}
}
