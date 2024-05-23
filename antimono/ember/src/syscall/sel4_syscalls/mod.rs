use anti_frame::{
    cpu::{set_tls, UserContext},
    early_print, early_println,
    vm::VmIo,
};

use super::SyscallReturn;
use crate::{
    log_syscall_entry,
    sel4::SeL4Regs,
    syscall::{SEL4_SET_TLS_BASE, SEL4_SYS_DEBUG_HALT, SEL4_SYS_DEBUG_PUT_CHAR},
    Result,
};

pub fn sel4_set_tls_base(user_context: &mut UserContext) -> Result<SyscallReturn> {
    log_syscall_entry!(SEL4_SET_TLS_BASE);
    let regs = user_context.general_regs_mut();

    let tls_base = regs.get_cap_reg();
    regs.set_tls(tls_base);
    set_tls(tls_base as u64);
    // todo change this
    Ok(SyscallReturn::Return(0))
}

// TODO may change this
pub fn sel4_kernel_putchar(user_context: &mut UserContext) -> Result<SyscallReturn> {
    log_syscall_entry!(SEL4_SYS_DEBUG_PUT_CHAR);
    let regs = user_context.general_regs_mut();
    let c = regs.get_cap_reg() as u8 as char;
    early_print!("{c}");
    Ok(SyscallReturn::Return(0))
}

/// use this to debug a syscall
pub fn sel4_sys_debug_halt() -> Result<SyscallReturn> {
    todo!()
    // log_syscall_entry!(SEL4_SYS_DEBUG_HALT);
    // let info = unsafe { &*(rootserver.boot_info as *const seL4_BootInfo) };
    // early_println!("in syscall info: {:?}", info.extraLen);
    // Ok(SyscallReturn::Return(0))
}
