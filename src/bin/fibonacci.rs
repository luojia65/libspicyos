#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos as std;

#[export_name = "main"]
fn main() {
    println!("Start fibonacci!");
    let n = 10;
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        a += b;
        core::mem::swap(&mut a, &mut b);
    }
    // 答案是55
    println!("Fibonacci sequence #{} is {}", n, a);
}
