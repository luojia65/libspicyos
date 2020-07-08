#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

pub mod syscall;

use core::alloc::Layout;
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;

/// 每个用户进程所用的堆大小（1M）
pub const USER_HEAP_SIZE: usize = 0x10_0000;

/// 大小为 [`USER_HEAP_SIZE`] 的堆空间
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

/// 使用 `buddy_system_allocator` 中的堆
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 打印 panic 信息并退出用户程序
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 程序入口
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    extern {
        fn main();
    }
    unsafe { main(); }
    syscall::process_exit(0)
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
