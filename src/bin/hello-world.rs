#![no_std]
#![no_main]

use libspicyos;

#[no_mangle]
pub fn main() -> usize {
    libspicyos::syscall::process_exit(99999999);
    // println!("Hello world from user mode program!");
    0
}

