#![no_std]
#![feature(start)]

#[no_mangle]
#[start]
fn start() -> () {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
