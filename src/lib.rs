#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

extern crate alloc;

pub mod fs;
pub mod process;

// 只用宏，必须隐藏。不private是因为里面有宏需要调用的函数
#[doc(hidden)]
#[macro_use]
pub mod stdio;

use buddy_system_allocator::LockedHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;

/// 每个用户进程所用的堆大小（1M）
pub const USER_HEAP_SIZE: usize = 0x10_0000;

/// 大小为 [`USER_HEAP_SIZE`] 的堆空间
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

/// 使用 `buddy_system_allocator` 中的堆
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 打印 panic 信息并退出用户程序
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    process::abort()
}

/// 程序入口
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    extern "C" {
        fn main();
    }
    // 运行用户提供的main函数
    unsafe {
        main(); // 暂时没有返回值，未来可以做Termination::report
    }
    process::exit(0)
}

/// 终止程序
#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort");
}

/// 内存不足时终止程序
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}
