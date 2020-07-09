const MODULE_FS: usize = 0xF0114514;

const FUNCTION_FS_READ: usize = 0x10002000;
const FUNCTION_FS_WRITE: usize = 0x30004000;

pub type Result<T> = core::result::Result<T, ()>;

pub fn fs_read(fd: usize, buf: &mut [u8]) -> Result<usize> {
    let (ans, err) = syscall(
        MODULE_FS,
        FUNCTION_FS_READ,
        fd,
        buf.as_ptr() as usize,
        buf.len(),
    );
    if err != 0 {
        return Err(()); // todo: an enum
    }
    return Ok(ans);
}

pub fn fs_write(fd: usize, buf: &[u8]) -> Result<usize> {
    let (ans, err) = syscall(
        MODULE_FS,
        FUNCTION_FS_WRITE,
        fd,
        buf.as_ptr() as usize,
        buf.len(),
    );
    if err != 0 {
        return Err(()); // todo: an enum
    }
    return Ok(ans);
}

fn syscall(
    module: usize,
    function: usize,
    param0: usize,
    param1: usize,
    param2: usize,
) -> (usize, usize) {
    let ans: usize;
    let err: usize;
    unsafe {
        llvm_asm!(
            "ecall"
            :"={a0}"(ans),"={a1}"(err)
            :"{a0}"(module),"{a1}"(function),"{a2}"(param0),"{a3}"(param1),"{a4}"(param2)
        );
    }
    (ans, err)
}
