#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos as std;

#[export_name = "main"]
fn main() {
    println!("Start fibonacci!");
    let n = 1000; 
    // 最后会产生panic："panicked at 'attempt to add with overflow'"
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        a += b;
        core::mem::swap(&mut a, &mut b);
    }
    println!("Fibonacci sequence #{} is {}", n, a);
}
