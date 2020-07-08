// param: a0: module, a1: function, a2...: params
// return: a0: value
pub fn syscall(module: usize, function: usize, param1: usize, param2: usize) -> usize {
    let ans: usize;
    unsafe { llvm_asm!(
        "ecall"
        :"={a0}"(ans)
        :"{a0}"(module),"{a1}"(function),"{a2}"(param1),"{a3}"(param2)
    ); }
    ans
}
