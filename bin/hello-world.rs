#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    0
}
