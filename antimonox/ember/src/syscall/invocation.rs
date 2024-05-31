pub mod decode;
mod invoke_tcb;
pub mod invoke_cnode;
pub mod invoke_untyped;
mod invoke_mmu_op;
pub mod invoke_irq;

use core::intrinsics::unlikely;

use crate::common::{structures::exception_t, message_info::seL4_MessageInfo_t, fault::seL4_Fault_t};
use log::debug;
use crate::task_manager::{get_currenct_thread, msgInfoRegister, capRegister, ThreadState, set_thread_state, n_msgRegisters};

use crate::kernel::boot::current_fault;
use crate::syscall::invocation::decode::decode_invocation;
use crate::syscall::{handle_fault, lookup_extra_caps_with_buf};
use crate::syscall::syscall_reply::{reply_error_from_kernel, reply_success_from_kernel};

#[no_mangle]
pub fn handleInvocation(isCall: bool, isBlocking: bool) -> exception_t {
    let thread = get_currenct_thread();
    let info = seL4_MessageInfo_t::from_word_security(thread.get_register(msgInfoRegister));
    let cptr = thread.get_register(capRegister);
    let lu_ret = thread.lookup_slot(cptr);
    if unlikely(lu_ret.status != exception_t::EXCEPTION_NONE) {
        debug!("Invocation of invalid capability {:#x}.", cptr);
        unsafe {
            current_fault = seL4_Fault_t::new_cap_fault(cptr, 0);
        }
        if isBlocking {
            handle_fault(thread);
        }
        return exception_t::EXCEPTION_NONE;
    }
    let buffer = thread.lookup_ipc_buffer(false);
    let status = lookup_extra_caps_with_buf(thread, buffer);
    if unlikely(status != exception_t::EXCEPTION_NONE) {
        debug!("Lookup of extra caps failed.");
        if isBlocking {
            // handleFault(thread);
            handle_fault(thread);
        }
        return exception_t::EXCEPTION_NONE;
    }

    let mut length = info.get_length();
    if unlikely(length > n_msgRegisters && buffer.is_none()) {
        length = n_msgRegisters;
    }

    let cap = unsafe {(*(lu_ret.slot)).cap};
    let status = decode_invocation(
        info.get_label(),
        length,
        unsafe {&mut *lu_ret.slot },
        &cap,
        cptr,
        isBlocking,
        isCall,
        buffer
    );
    if status == exception_t::EXCEPTION_PREEMTED {
        return status;
    }

    if status == exception_t::EXCEPTION_SYSCALL_ERROR {
        if isCall {
            reply_error_from_kernel(thread);
        }
        return exception_t::EXCEPTION_NONE;
    }

    if unlikely(thread.get_state() == ThreadState::ThreadStateRestart) {
        if isCall {
            reply_success_from_kernel(thread);
        }
        set_thread_state(thread, ThreadState::ThreadStateRunning);
    }
    return exception_t::EXCEPTION_NONE;
}