#![no_std]
#![no_main]

#[macro_use]
extern crate libspicyos as std;

#[export_name = "main"]
fn main() {
    // 这些是unicode字符，有什么就输出什么，不应该输出乱码
    println!("你们可能不知道只用20万赢到578万是什么概念。我们一般只会用两个字来形容这种人：赌怪")
}
