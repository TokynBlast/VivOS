#![no_std]
#![no_main]

#[entry]
fn main() -> Status {

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  println!("FATAL ERROR, TRY AGAIN!\n{}", _info);
  loop {}
}
