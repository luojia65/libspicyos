#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos;
// use libspicyos as std;

#[export_name = "main"]
fn main() {
    println!("你们可能不知道只用20万赢到578万是什么概念。我们一般只会用两个字来形容这种人：赌怪")
}
