use anti_frame::cpu::{set_tls, UserContext};

use super::SyscallReturn;
use crate::{error::Result, log_syscall_entry, sel4::SeL4Regs, syscall::SEL4_SET_TLS_BASE};

pub fn sel4_set_tls_base(user_context: &mut UserContext) -> Result<SyscallReturn> {
    log_syscall_entry!(SEL4_SET_TLS_BASE);
    let regs = user_context.general_regs_mut();

    let tls_base = regs.get_cap_reg();
    regs.set_tls(tls_base);
    set_tls(tls_base as u64);
    // todo change this
    Ok(SyscallReturn::Return(0))
}
