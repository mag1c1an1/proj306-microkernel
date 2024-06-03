use aster_frame::{
    cpu::UserContext,
    early_print,
    vm::VmIo,
};

use crate::{EmberResult, log_syscall_entry};
use crate::sel4::SeL4ABI;
use crate::sel4::syscall_id::*;
use crate::syscall::SyscallReturn;

pub mod invocation;

/// TODO find out
pub fn sel4_set_tls_base(user_context: &mut UserContext) -> EmberResult<SyscallReturn> {
    log_syscall_entry!(SetTLSBase);
    let tls_base = user_context.cap_reg();
    user_context.set_tls(tls_base as u64);
    Ok(SyscallReturn::Return(0))
}

// TODO may change this
pub fn sel4_kernel_putchar(user_context: &mut UserContext) -> EmberResult<SyscallReturn> {
    log_syscall_entry!(DebugPutChar);
    let c = user_context.cap_reg() as u8 as char;
    early_print!("{}", c);
    Ok(SyscallReturn::Return(0))
}
//
// /// use this to debug a syscall
// pub fn sel4_sys_debug_halt() -> Result<SyscallReturn> {
//     todo!()
//     // log_syscall_entry!(SEL4_SYS_DEBUG_HALT);
//     // let info = unsafe { &*(rootserver.boot_info as *const seL4_BootInfo) };
//     // early_println!("in syscall info: {:?}", info.extraLen);
//     // Ok(SyscallReturn::Return(0))
// }
// pub fn sel4_debug_dump_scheduler() {}
//
// pub fn sel4_debug_snapshot() {}
//
// pub fn sel4_debug_cap_identify() {}
//
// pub fn sel4_debug_name_thread() {}
//
// pub fn sel4_send() {}
// pub fn sel4_nb_send() {}
// pub fn sel4_call() {}
// pub fn sel4_recv() {}
// pub fn sel4_reply() {}
// pub fn sel4_reply_recv() {}
// pub fn sel4_nb_recv() {}
// pub fn sel4_yield() {}
