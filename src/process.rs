use crate::syscall::syscall;

const MODULE_PROCESS: usize = 0x23336666;

const FUNCTION_PROCESS_EXIT: usize = 0x99998888;
const FUNCTION_PROCESS_GET_ID: usize = 0x77776666;

pub fn exit(code: i32) -> ! {
    let code = u32::from_ne_bytes(code.to_ne_bytes()) as usize;
    syscall(MODULE_PROCESS, FUNCTION_PROCESS_EXIT, code, 0);
    unreachable!()
}

pub fn id() -> u32 {
    syscall(MODULE_PROCESS, FUNCTION_PROCESS_GET_ID, 0, 0) as u32
}
