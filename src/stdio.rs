use core::fmt::{self, Write};

// const STDIN: usize = 0;
const STDOUT: usize = 1;

/// 实现 [`core::fmt::Write`] trait 来进行格式化输出
struct Stdout;

impl Write for Stdout {
    /// 打印一个字符串
    fn write_str(&mut self, s: &str) -> fmt::Result {
        crate::fs::fs_write(STDOUT, s.as_bytes()).unwrap();
        Ok(())
    }
}

// 打印由 [`core::format_args!`] 格式化后的数据
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类似于标准库中的 `print!` 宏
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::_print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
