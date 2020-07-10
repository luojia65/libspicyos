//! 进程相关的API

const MODULE_PROCESS: usize = 0x23336666;

const FUNCTION_PROCESS_EXIT: usize = 0x99998888;
const FUNCTION_PROCESS_GET_ID: usize = 0x77776666;

/// 以给定的代码退出
pub fn exit(code: i32) -> ! {
    let code = u32::from_ne_bytes(code.to_ne_bytes()) as usize;
    syscall(MODULE_PROCESS, FUNCTION_PROCESS_EXIT, code);
    unreachable!()
}

/// 以异常代码退出
pub fn abort() -> ! {
    exit(-1)
}

/// 得到当前进程的编号
pub fn id() -> u32 {
    syscall(MODULE_PROCESS, FUNCTION_PROCESS_GET_ID, 0) as u32
}

fn syscall(module: usize, function: usize, param0: usize) -> usize {
    let ans: usize;
    unsafe {
        llvm_asm!(
            "ecall"
            :"={a0}"(ans)
            :"{a0}"(module),"{a1}"(function),"{a2}"(param0)
        );
    }
    ans
}
