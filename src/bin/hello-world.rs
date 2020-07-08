#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos;
use libspicyos as std;

#[export_name = "main"]
fn main() {
    println!("Hello world!");
    println!("I am process #{}!", std::process::id());
    println!("Exit with code 666!");
    std::process::exit(666)
}
