#![no_std]
#![no_main]

use libspicyos as std;

#[export_name = "main"]
fn main() {
    let id = std::process::id();
    let exit_code = i32::from_ne_bytes(id.to_ne_bytes());
    std::process::exit(exit_code);
}

