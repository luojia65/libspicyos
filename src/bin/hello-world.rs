#![no_std]
#![no_main]

use libspicyos;

#[export_name = "main"]
fn main() {
    libspicyos::syscall::process_exit(99999999);
}

