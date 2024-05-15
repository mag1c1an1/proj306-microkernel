use core::arch::asm;
use log::debug;
use crate::{
    config::{
        RISCVInstructionAccessFault, RISCVInstructionPageFault, RISCVLoadAccessFault,
        RISCVLoadPageFault, RISCVStoreAccessFault, RISCVStorePageFault,
    },
    syscall::slowpath
};

use crate::task_manager::*;
use crate::exception::{handleUserLevelFault, handleVMFaultEvent};

use crate::interrupt::handler::handleInterruptEntry;

#[cfg(feature = "ENABLE_SMP")]
use crate::{
    common::utils::cpu_id, interrupt::getActiveIRQ,
    deps::{clh_is_self_in_queue, clh_lock_release, clh_lock_acquire}
};

#[no_mangle]
pub fn restore_user_context() {
}



#[no_mangle]
pub fn c_handle_interrupt() {
    // debug!("c_handle_interrupt");
    // if hart_id() != 0 {
    //     debug!("c_handle_interrupt");
    // }
    #[cfg(feature = "ENABLE_SMP")] {
        use crate::config::IRQConst::INTERRUPT_IPI_0;
        if getActiveIRQ() != INTERRUPT_IPI_0 as usize {
            unsafe { clh_lock_acquire(cpu_id(), true); }
        }
    }
    #[cfg(feature = "ENABLE_UINTC")]
    crate::uintc::uintr_save();
    // debug!("c_handle_interrupt");
    handleInterruptEntry();
    restore_user_context();
}

#[no_mangle]
pub fn c_handle_exception() {
    #[cfg(feature = "ENABLE_SMP")]
    unsafe { clh_lock_acquire(cpu_id(), false); }
    // if hart_id() == 0 {
    //     debug!("c_handle_exception");
    // }
    #[cfg(feature = "ENABLE_UINTC")]
    crate::uintc::uintr_save();

    let cause = 0;
    debug!("{:?}", get_currenct_thread().tcbArch);
    debug!("c_handle_exception, cause: {}", cause);
    // debug!("handle_fault: {}", cause);
    match cause {
        RISCVInstructionAccessFault
        | RISCVLoadAccessFault
        | RISCVStoreAccessFault
        | RISCVLoadPageFault
        | RISCVStorePageFault
        | RISCVInstructionPageFault => {
            handleVMFaultEvent(cause);
        }
        _ => {
            handleUserLevelFault(cause, 0);
        }
    }
    restore_user_context();
}

#[no_mangle]
pub fn c_handle_syscall(_cptr: usize, _msgInfo: usize, syscall: usize) {
    #[cfg(feature = "ENABLE_SMP")]
    unsafe { clh_lock_acquire(cpu_id(), false); }
    // if hart_id() == 0 {
    //     debug!("c_handle_syscall: syscall: {},", syscall as isize);
    // }
    #[cfg(feature = "ENABLE_UINTC")]
    crate::uintc::uintr_save();
    slowpath(syscall);
    // debug!("c_handle_syscall complete");
}
