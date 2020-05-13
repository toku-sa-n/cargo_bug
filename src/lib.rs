#![no_std]

extern crate rlibc;

const TWO_DIMENSIONAL_ARRAY: [[char; WIDTH]; HEIGHT] = [
    ['1', '2', '3', '4', '5', '6'],
    ['1', '2', '3', '4', '5', '6'],
    ['1', '2', '3', '4', '5', '6'],
    ['1', '2', '3', '4', '5', '6'],
    ['1', '2', '3', '4', '5', '6'],
    ['1', '2', '3', '4', '5', '6'],
];

const WIDTH: usize = 6;
const HEIGHT: usize = 6;

fn foo(_arr: [[char; WIDTH]; HEIGHT]) -> () {
    loop {}
}

#[no_mangle]
fn _start() {
    foo(TWO_DIMENSIONAL_ARRAY);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
