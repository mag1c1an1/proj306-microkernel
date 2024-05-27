use crate::common::message_info::seL4_MessageInfo_t;
use crate::task_manager::{msgInfoRegister, tcb_t};
use crate::common::sel4_config::*;
use crate::config::badgeRegister;
use crate::kernel::boot::{current_lookup_fault, current_syscall_error};

#[inline]
pub fn reply_error_from_kernel(thread: &mut tcb_t) {
    thread.set_register(badgeRegister, 0);
    unsafe {
        let len = set_mrs_for_syscall_error(thread);
        thread.set_register(msgInfoRegister, seL4_MessageInfo_t::new(current_syscall_error._type, 0, 0, len).to_word());
    }
}

#[inline]
pub fn reply_success_from_kernel(thread: &mut tcb_t) {
    thread.set_register(badgeRegister, 0);
    thread.set_register(msgInfoRegister, seL4_MessageInfo_t::new(0, 0, 0, 0).to_word());
}

pub unsafe fn set_mrs_for_syscall_error(thread: &mut tcb_t) -> usize {
    match current_syscall_error._type {
        seL4_InvalidArgument => thread.set_mr(0, current_syscall_error.invalidArgumentNumber),
        seL4_InvalidCapability => thread.set_mr(0, current_syscall_error.invalidCapNumber),
        seL4_RangeError => {
            thread.set_mr(0, current_syscall_error.rangeErrorMin);
            thread.set_mr(1, current_syscall_error.rangeErrorMax)
        }
        seL4_FailedLookup => {
            let flag = current_syscall_error.failedLookupWasSource == 1;
            thread.set_mr(0, flag as usize);
            return thread.set_lookup_fault_mrs(1, &current_lookup_fault);
        }
        seL4_IllegalOperation
            | seL4_AlignmentError
            | seL4_TruncatedMessage
            | seL4_DeleteFirst
            | seL4_RevokeFirst => 0,
        seL4_NotEnoughMemory => thread.set_mr(0, current_syscall_error.memoryLeft),
        _ => {
            panic!("invalid syscall error")
        }
    }
}
