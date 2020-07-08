// param: a0: module, a1: function, a2...: params
// return: a0: value
fn syscall(mut module: usize, function: usize, param1: usize, param2: usize) -> usize {
    unsafe { llvm_asm!(
        "ecall"
        :"={a0}"(module)
        :"{a1}"(function),"{a2}"(param1),"{a3}"(param2)
    ); }
    module
}

const MODULE_PROCESS: usize = 0x23336666;
const FUNCTION_PROCESS_EXIT: usize = 0x99998888;

pub fn process_exit(code: i32) -> ! {
    let code = u32::from_ne_bytes(code.to_ne_bytes()) as usize;
    syscall(MODULE_PROCESS, FUNCTION_PROCESS_EXIT, code, 0);
    unreachable!()
}
