// use crate::{error::Errno, return_errno_with_message, Result};

pub mod sel4_syscalls;
// pub mod utils;
// pub mod invocation;
// pub mod syscall_reply;

// use core::intrinsics::unlikely;
// use log::debug;
// use crate::common::fault::{FaultType, lookup_fault_t, seL4_Fault_t};
// use crate::common::sel4_config::tcbCaller;

// pub const SysCall: isize = -1;
// pub const SysReplyRecv: isize = -2;
// pub const SysSend: isize = -3;
// pub const SysNBSend: isize = -4;
// pub const SysRecv: isize = -5;
// pub const SysReply: isize = -6;
// pub const SysYield: isize = -7;
// pub const SysNBRecv: isize = -8;
// pub const SysWakeSyscallHandler: isize = -16;
// use crate::common::structures::exception_t;
// use crate::common::utils::convert_to_mut_type_ref;
// use crate::cspace::interface::CapTag;
// use crate::deps::handleUnknownSyscall;
// use crate::task_manager::{schedule, activateThread, tcb_t, set_thread_state, ThreadState, get_currenct_thread, capRegister, rescheduleRequired};
// use crate::task_manager::ipc::{endpoint_t, notification_t};
// pub use utils::*;

// use crate::{kernel::c_traps::restore_user_context, config::irqInvalid, interrupt::getActiveIRQ};
// use crate::interrupt::handler::handleInterrupt;
// use crate::kernel::boot::{current_fault, current_lookup_fault};

// use self::invocation::handleInvocation;

// // use crate::async_runtime::{coroutine_run_until_blocked, coroutine_wake, NEW_BUFFER_MAP, NewBuffer};
// use core::sync::atomic::Ordering::SeqCst;

// #[no_mangle]
// pub fn slowpath(syscall: usize) {
//     // debug!("enter slow path: {}", syscall as isize);
//     if (syscall as isize) < -8 || (syscall as isize) > -1 {
//         if (syscall as isize) == SysWakeSyscallHandler {
//             wake_syscall_handler();
//         } else {
//             unsafe {
//                 handleUnknownSyscall(syscall);
//             }
//         }
//     } else {
//         handleSyscall(syscall);
//     }
//     restore_user_context();
// }

// #[no_mangle]
// pub fn handleSyscall(_syscall: usize) -> exception_t {
//     let syscall: isize = _syscall as isize;
//     // if hart_id() == 0 {
//     //     debug!("handle syscall: {}", syscall);
//     // }
//     match syscall {
//         SysSend => {
//             let ret = handleInvocation(false, true);

//             if unlikely(ret != exception_t::EXCEPTION_NONE) {
//                 let irq = getActiveIRQ();
//                 if irq != irqInvalid {
//                     handleInterrupt(irq);
//                 }
//             }
//         }
//         SysNBSend => {
//             let ret = handleInvocation(false, false);
//             if unlikely(ret != exception_t::EXCEPTION_NONE) {
//                 let irq = getActiveIRQ();
//                 if irq != irqInvalid {
//                     handleInterrupt(irq);
//                 }
//             }
//         }
//         SysCall => {
//             let ret = handleInvocation(true, true);
//             if unlikely(ret != exception_t::EXCEPTION_NONE) {
//                 let irq = getActiveIRQ();
//                 if irq != irqInvalid {
//                     handleInterrupt(irq);
//                 }
//             }
//         }
//         SysRecv => {
//             handle_recv(true);
//         }
//         SysReply => handle_reply(),
//         SysReplyRecv => {
//             handle_reply();
//             handle_recv(true);
//         }
//         SysNBRecv => handle_recv(false),
//         SysYield => handle_yield(),
//         _ => panic!("Invalid syscall"),
//     }
//     schedule();
//     activateThread();
//     exception_t::EXCEPTION_NONE
// }

// fn send_fault_ipc(thread: &mut tcb_t) -> exception_t {
//     let origin_lookup_fault = unsafe { current_lookup_fault };
//     let lu_ret = thread.lookup_slot(thread.tcbFaultHandler);
//     if lu_ret.status != exception_t::EXCEPTION_NONE {
//         unsafe { current_fault = seL4_Fault_t::new_cap_fault(thread.tcbFaultHandler, 0); }
//         return exception_t::EXCEPTION_FAULT;
//     }
//     let handler_cap = &unsafe { (*lu_ret.slot).capability };
//     if handler_cap.get_cap_type() == CapTag::CapEndpointCap
//         && (handler_cap.get_ep_can_grant() != 0
//             || handler_cap.get_ep_can_grant_reply() != 0) {
//         thread.tcbFault = unsafe { current_fault };
//         if thread.tcbFault.get_fault_type() == FaultType::CapFault {
//             thread.tcbLookupFailure = origin_lookup_fault;
//         }
//         convert_to_mut_type_ref::<endpoint_t>(handler_cap.get_ep_ptr()).send_ipc(
//             thread,
//             true,
//             true,
//             handler_cap.get_ep_can_grant() != 0,
//             handler_cap.get_ep_badge(),
//             true,
//         );
//     } else {
//         unsafe {
//             current_fault = seL4_Fault_t::new_cap_fault(thread.tcbFaultHandler, 0);
//             current_lookup_fault = lookup_fault_t::new_missing_cap(0);
//         }
//         return exception_t::EXCEPTION_FAULT;
//     }
//     exception_t::EXCEPTION_NONE
// }

// #[inline]
// pub fn handle_fault(thread: &mut tcb_t) {
//     let fault = send_fault_ipc(thread);
//     if fault != exception_t::EXCEPTION_NONE {
//         debug!("send_fault_ipc fail: {:?}", fault);
//         set_thread_state(thread, ThreadState::ThreadStateInactive);
//     }
// }

// fn handle_reply() {
//     let current_thread = get_currenct_thread();
//     let caller_slot = current_thread.get_cspace_mut_ref(tcbCaller);
//     let caller_cap = &caller_slot.capability;
//     if caller_cap.get_cap_type() == CapTag::CapReplyCap {
//         if caller_cap.get_reply_master() != 0 {
//             return;
//         }
//         let caller = convert_to_mut_type_ref::<tcb_t>(caller_cap.get_reply_tcb_ptr());
//         current_thread.do_reply(caller, caller_slot, caller_cap.get_reply_can_grant() != 0);
//     }
// }

// fn handle_recv(block: bool) {
//     let current_thread = get_currenct_thread();
//     let ep_cptr = current_thread.get_register(capRegister);
//     let lu_ret = current_thread.lookup_slot(ep_cptr);
//     if lu_ret.status != exception_t::EXCEPTION_NONE {
//         unsafe { current_fault = seL4_Fault_t::new_cap_fault(ep_cptr, 1); }
//         return handle_fault(current_thread);
//     }
//     let ipc_cap = unsafe { (*lu_ret.slot).capability };
//     match ipc_cap.get_cap_type() {
//         CapTag::CapEndpointCap => {
//             if unlikely(ipc_cap.get_ep_can_receive() == 0) {
//                 unsafe {
//                     debug!("handle recv fault");
//                     current_lookup_fault = lookup_fault_t::new_missing_cap(0);
//                     current_fault = seL4_Fault_t::new_cap_fault(ep_cptr, 1);
//                 }
//                 return handle_fault(current_thread);
//             }
//             current_thread.delete_caller_cap();
//             convert_to_mut_type_ref::<endpoint_t>(ipc_cap.get_ep_ptr()).receive_ipc(
//                 current_thread,
//                 block,
//                 ipc_cap.get_ep_can_grant() != 0
//             );
//         }

//         CapTag::CapNotificationCap => {
//             let ntfn = convert_to_mut_type_ref::<notification_t>(ipc_cap.get_nf_ptr());
//             let bound_tcb_ptr = ntfn.get_bound_tcb();
//             if unlikely(ipc_cap.get_nf_can_receive() == 0 || (bound_tcb_ptr != 0 && bound_tcb_ptr != current_thread.get_ptr())) {
//                 unsafe {
//                     current_lookup_fault = lookup_fault_t::new_missing_cap(0);
//                     current_fault = seL4_Fault_t::new_cap_fault(ep_cptr, 1);
//                 }
//                 return handle_fault(current_thread);
//             }
//             return ntfn.receive_signal(current_thread, block)
//         }
//         _ => {
//             unsafe {
//                 current_lookup_fault = lookup_fault_t::new_missing_cap(0);
//                 current_fault = seL4_Fault_t::new_cap_fault(ep_cptr, 1);
//             }
//             return handle_fault(current_thread);
//         }
//     }
// }

// fn handle_yield() {
//     get_currenct_thread().sched_dequeue();
//     get_currenct_thread().sched_append();
//     rescheduleRequired();
// }

// fn wake_syscall_handler() {
//     debug!("wake_syscall_handler: enter");
//     // if let Some(cid) = get_currenct_thread().asyncSysHandlerCid {
//     //     debug!("wake_syscall_handler: current thread's handler cid: {:?}", cid);
//     //     for item in unsafe { &NEW_BUFFER_MAP } {
//     //         if item.cid.0 == cid{
//     //             let new_buffer = item.buf;
//     //             if new_buffer.recv_req_status.load(SeqCst) {
//     //                 debug!("wake_syscall_handler: wake cid: {}", item.cid.0);
//     //                 coroutine_wake(&item.cid);
//     //             }
//     //             break;
//     //         }
//     //     }
//     // }
// }

// use anti_frame::cpu::UserContext;

// use self::sel4_syscalls::{sel4_kernel_putchar, sel4_set_tls_base, sel4_sys_debug_halt};

macro_rules! define_syscall_nums {
    ( $( $name: ident = $num: expr ),+ ) => {
        $(
            const $name: i64  = $num;
        )*
    }
}

/// This macro is used to define syscall handler.
/// The first param is ths number of parameters,
/// The second param is the function name of syscall handler,
/// The third is optional, means the args(if parameter number > 0),
/// The third is optional, means if cpu context is required.
macro_rules! syscall_handler {
    (0, $fn_name: ident) => { $fn_name() };
    (0, $fn_name: ident, $context: expr) => { $fn_name($context) };
    (1, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _) };
    (1, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $context) };
    (2, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _, $args[1] as _)};
    (2, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $args[1] as _, $context)};
    (3, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _)};
    (3, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $context)};
    (4, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _)};
    (4, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _), $context};
    (5, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _, $args[4] as _)};
    (5, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _, $args[4] as _, $context)};
    (6, $fn_name: ident, $args: ident) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _, $args[4] as _, $args[5] as _)};
    (6, $fn_name: ident, $args: ident, $context: expr) => { $fn_name($args[0] as _, $args[1] as _, $args[2] as _, $args[3] as _, $args[4] as _, $args[5] as _, $context)};
}
// pub struct SyscallArgument {
//     syscall_number: i64,
//     args: [u64; 6],
// }
//
// impl SyscallArgument {
//     fn new_from_context(context: &UserContext) -> Self {
//         let syscall_number = context.rax() as i64;
//         let mut args = [0u64; 6];
//         args[0] = context.rdi() as u64;
//         args[1] = context.rsi() as u64;
//         args[2] = context.rdx() as u64;
//         args[3] = context.r10() as u64;
//         args[4] = context.r8() as u64;
//         args[5] = context.r9() as u64;
//         Self {
//             syscall_number,
//             args,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy)]
// pub enum SyscallReturn {
//     /// return isize, this value will be used to set rax
//     Return(isize),
//     /// does not need to set rax
//     NoReturn,
// }
//
// pub fn handle_syscall(context: &mut UserContext) {
//     let syscall_frame = SyscallArgument::new_from_context(context);
//     let syscall_return =
//         syscall_dispatch(syscall_frame.syscall_number, syscall_frame.args, context);
//
//     match syscall_return {
//         Ok(return_value) => {
//             if let SyscallReturn::Return(return_value) = return_value {
//                 context.set_rax(return_value as usize);
//             }
//         }
//         Err(err) => {
//             debug!("syscall return error: {:?}", err);
//             let errno = err.error() as i32;
//             context.set_rax((-errno) as usize)
//         }
//     }
// }
//
// define_syscall_nums! {
//     SEL4_SYS_CALL = -1,
//     SEL4_SYS_REPLY_RECV = -2,
//     SEL4_SYS_SEND = -3,
//     SEL4_SYS_NB_SEND = -4,
//     SEL4_SYS_RECV = -5,
//     SEL4_SYS_REPLY = -6,
//     SEL4_SYS_YIELD= -7,
//     SEL4_SYS_NB_RECV = -8,
//     SEL4_SYS_DEBUG_PUT_CHAR = -9,
//     SEL4_SYS_DEBUG_DUMP_SCHEDULER=-10,
//     SEL4_SYS_DEBUG_HALT = -11,
//     SEL4_SYS_DEBUG_CAP_IDENTIFY = -12,
//     SEL4_SYS_DEBUG_SNAPSHOT_RESTORE = -13,
//     SEL4_SYS_DEBUG_NAME_THREAD = -14,
//     SEL4_SYS_DEBUG_SEND_IPI = -15,
//     SEL4_SET_TLS_BASE = -29
// }
//
// pub fn syscall_dispatch(
//     syscall_number: i64, // sel4 only
//     args: [u64; 6],
//     context: &mut UserContext,
// ) -> Result<SyscallReturn> {
//     match syscall_number {
//         SEL4_SET_TLS_BASE => syscall_handler!(0, sel4_set_tls_base, context),
//         SEL4_SYS_DEBUG_PUT_CHAR => syscall_handler!(0, sel4_kernel_putchar, context),
//         SEL4_SYS_DEBUG_HALT => syscall_handler!(0, sel4_sys_debug_halt),
//         // SEL4_SYS_DEBUG_NAME_THREAD => syscall_handler!(1, sel4_sys_debug_name_thread),
//         _ => {
//             error!("Unimplemented syscall number: {}", syscall_number);
//             return_errno_with_message!(Errno::ENOSYS, "Unimplemented syscall");
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! log_syscall_entry {
//     ($syscall_name: tt) => {
//         if log_enabled!(log::Level::Info) {
//             let syscall_name_str = stringify!($syscall_name);
//             // let pid = $crate::current!().pid();
//             // let tid = $crate::current_thread!().tid();
//             info!(
//                 // "[pid={}][tid={}][id={}][{}]",
//                 "[id={}][{}]",
//                 $syscall_name, syscall_name_str
//             );
//         }
//     };
// }
