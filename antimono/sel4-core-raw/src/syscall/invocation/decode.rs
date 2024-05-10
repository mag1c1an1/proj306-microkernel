mod decode_tcb_invocation;
mod decode_domain_invocation;
mod decode_cnode_invocation;
pub mod decode_untyped_invocation;
mod decode_mmu_invocation;
pub mod decode_irq_invocation;

use alloc::boxed::Box;
use core::{alloc, intrinsics::unlikely};

use crate::common::{structures::{exception_t, seL4_IPCBuffer}, sel4_config::seL4_InvalidCapability, utils::convert_to_mut_type_ref, message_info::MessageLabel};
use crate::cspace::interface::{cte_t, cap_t, CapTag};
use crate::task_manager::ipc::{endpoint_t, notification_t};
use log::debug;
use crate::async_runtime::NewBuffer;
use crate::common::sel4_config::seL4_TruncatedMessage;
use crate::task_manager::{set_thread_state, get_currenct_thread, ThreadState, tcb_t};

use crate::kernel::boot::{current_syscall_error, get_extra_cap_by_index};
use crate::syscall::invocation::decode::decode_irq_invocation::decode_irq_handler_invocation;

use self::{
    decode_tcb_invocation::decode_tcb_invocation,
    decode_domain_invocation::decode_domain_invocation,
    decode_cnode_invocation::decode_cnode_invocation,
    decode_untyped_invocation::decode_untyed_invocation,
    decode_mmu_invocation::decode_mmu_invocation,
    decode_irq_invocation::decode_irq_control_invocation,
};


pub fn decode_invocation(label: MessageLabel, length: usize, slot: &mut cte_t, cap: &cap_t, cap_index: usize,
                        block: bool, call: bool, buffer: Option<&seL4_IPCBuffer>) -> exception_t {
    match cap.get_cap_type() {
        CapTag::CapNullCap | CapTag::CapZombieCap  => {
            debug!("Attempted to invoke a null or zombie cap {:#x}, {:?}.", cap_index, cap.get_cap_type());
            unsafe {
                current_syscall_error._type = seL4_InvalidCapability;
                current_syscall_error.invalidCapNumber = 0;
            }
            return exception_t::EXCEPTION_SYSCALL_ERROR;
        }

        CapTag::CapEndpointCap => {
            if unlikely(cap.get_ep_can_send() == 0) {
                debug!("Attempted to invoke a read-only endpoint cap {}.", cap_index);
                unsafe {
                    current_syscall_error._type = seL4_InvalidCapability;
                    current_syscall_error.invalidCapNumber = 0;
                }
                return exception_t::EXCEPTION_SYSCALL_ERROR;
            }
            set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
            convert_to_mut_type_ref::<endpoint_t>(cap.get_ep_ptr()).send_ipc(get_currenct_thread(),
                                                                             block,
                                                                             call, cap.get_ep_can_grant() != 0,
                                                                             cap.get_ep_badge(),
                                                                             cap.get_ep_can_grant_reply() != 0);
            return exception_t::EXCEPTION_NONE;
        }

        CapTag::CapNotificationCap => {
            #[cfg(feature = "ENABLE_UINTC")]
            {
                if label == MessageLabel::UintrRegisterReceiver {
                    use crate::task_manager::ipc::notification_t;
                    let tcb_slot = get_extra_cap_by_index(0);
                    if tcb_slot.is_none() {
                        debug!("UInt RegisterReceiver: Truncated message.");
                        unsafe { current_syscall_error._type = seL4_TruncatedMessage; }
                        return exception_t::EXCEPTION_SYSCALL_ERROR;
                    }
                    let tcb_cap = tcb_slot.unwrap().cap;
                    crate::uintc::register_receiver(convert_to_mut_type_ref::<notification_t>(cap.get_nf_ptr()), convert_to_mut_type_ref::<tcb_t>(tcb_cap.get_tcb_ptr()));
                    set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
                    return exception_t::EXCEPTION_NONE;
                } else if label == MessageLabel::UintrRegisterSender {
                    crate::uintc::register_sender(cap);
                    set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
                    return exception_t::EXCEPTION_NONE;
                } else if label == MessageLabel::UintrRegisterAsyncSyscall {
                    use crate::async_runtime::async_syscall_handler;
                    use crate::async_runtime::NEW_BUFFER_MAP;
                    use crate::async_runtime::{coroutine_spawn, NewBufferMap};
                    let new_buffer_slot = get_extra_cap_by_index(0);
                    if new_buffer_slot.is_none() {
                        debug!("UInt RegisterAsyncSyscall: Truncated message.");
                        unsafe { current_syscall_error._type = seL4_TruncatedMessage; }
                        return exception_t::EXCEPTION_SYSCALL_ERROR;
                    }
                    debug!("UintrRegisterAsyncSyscall: Enter");
                    let new_buffer_cap = new_buffer_slot.unwrap().cap;
                    //注册发送端，获取sender_id
                    let sender_id = crate::uintc::register_sender_async_syscall(cap);
                    debug!("UintrRegisterAsyncSyscall: sender id = {:?}", sender_id);
                    //生成异步系统调用处理协程并将cid保存至tcb
                    let cid = coroutine_spawn(Box::pin(async_syscall_handler(*cap, new_buffer_cap, get_currenct_thread(), sender_id as usize)));
                    get_currenct_thread().asyncSysHandlerCid = Some(cid.0);
                    debug!("UintrRegisterAsyncSyscall: coroutine id = {:?}", cid);
                    unsafe {
                        NEW_BUFFER_MAP.push(NewBufferMap {
                            buf: &mut *(new_buffer_cap.get_frame_base_ptr() as *mut NewBuffer),
                            cid,
                        })
                    }
                    set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
                    return exception_t::EXCEPTION_NONE;
                }
            }
            if unlikely(cap.get_nf_can_send() == 0) {
                debug!(
                    "Attempted to invoke a read-only notification cap {}.",
                    cap_index
                );
                unsafe {
                    current_syscall_error._type = seL4_InvalidCapability;
                    current_syscall_error.invalidCapNumber = 0;
                }
                return exception_t::EXCEPTION_SYSCALL_ERROR;
            }
            set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
            convert_to_mut_type_ref::<notification_t>(cap.get_nf_ptr()).send_signal(cap.get_nf_badge());
            exception_t::EXCEPTION_NONE
        }

        CapTag::CapReplyCap => {
            if unlikely(cap.get_reply_master() != 0) {
                debug!("Attempted to invoke an invalid reply cap {}.", cap_index);
                unsafe {
                    current_syscall_error._type = seL4_InvalidCapability;
                    current_syscall_error.invalidCapNumber = 0;
                    return exception_t::EXCEPTION_SYSCALL_ERROR;
                }
            }
            set_thread_state(get_currenct_thread(), ThreadState::ThreadStateRestart);
            get_currenct_thread().do_reply(
                convert_to_mut_type_ref::<tcb_t>(cap.get_reply_tcb_ptr()),
                slot,
                cap.get_reply_can_grant() != 0
            );
            exception_t::EXCEPTION_NONE
        }
        CapTag::CapThreadCap => decode_tcb_invocation(label, length, cap, slot, call, buffer),
        CapTag::CapDomainCap => decode_domain_invocation(label, length, buffer),
        CapTag::CapCNodeCap => decode_cnode_invocation(label, length, cap, buffer),
        CapTag::CapUntypedCap => decode_untyed_invocation(label, length, slot, cap, buffer),
        CapTag::CapIrqControlCap => decode_irq_control_invocation(label, length, slot, buffer),
        CapTag::CapIrqHandlerCap => decode_irq_handler_invocation(label, cap.get_irq_handler()),
        _ => decode_mmu_invocation(label, length, slot, call, buffer)
    }

}