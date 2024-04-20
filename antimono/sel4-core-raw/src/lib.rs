#![no_std]
#![allow(clippy::upper_case_acronyms)]
extern crate alloc;

// use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec;

use aster_frame::cpu::UserContext;
use aster_frame::prelude::*;
use aster_frame::task::{Task, TaskOptions};
use aster_frame::user::{UserEvent, UserMode, UserSpace};
use aster_frame::vm::{Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmPerm, VmSpace, PAGE_SIZE};
use align_ext::AlignExt;


pub mod object;
pub mod structures;

pub mod config;

pub mod machine;
pub mod model;
pub mod syscall;

pub mod error {
    #[repr(u32)]
    pub enum Exception {
        None = 0,
        Fault = 1,
        Lookup,
        SysCall,
        Preempted,
    }

    pub type Result<T, E = Exception> = core::result::Result<T, E>;
}

pub fn create_user_space(program: &[u8]) -> UserSpace {
    let user_pages = {
        let nframes = program.len().align_up(PAGE_SIZE) / PAGE_SIZE;
        let vm_frames = VmAllocOptions::new(nframes).alloc().unwrap();
        // Phyiscal memory pages can be only accessed
        // via the VmFrame abstraction.
        vm_frames.write_bytes(0, program).unwrap();
        vm_frames
    };
    let user_address_space = {
        const MAP_ADDR: Vaddr = 0x0040_0000; // The map addr for statically-linked executable

        // The page table of the user space can be
        // created and manipulated safely through
        // the VmSpace abstraction.
        let vm_space = VmSpace::new();
        let mut options = VmMapOptions::new();
        options.addr(Some(MAP_ADDR)).perm(VmPerm::RWX);
        vm_space.map(user_pages, &options).unwrap();
        vm_space
    };
    let user_cpu_state = {
        const ENTRY_POINT: Vaddr = 0x0040_1000; // The entry point for statically-linked executable

        // The user-space CPU states can be initialized
        // to arbitrary values via the UserContext
        // abstraction.
        let mut user_cpu_state = UserContext::default();
        user_cpu_state.set_rip(ENTRY_POINT);
        user_cpu_state
    };
    UserSpace::new(user_address_space, user_cpu_state)
}



pub fn create_user_task(user_space: Arc<UserSpace>) -> Arc<Task> {
    fn user_task() {
        let current = Task::current();
        // Switching between user-kernel space is
        // performed via the UserMode abstraction.
        let mut user_mode = {
            let user_space = current.user_space().unwrap();
            UserMode::new(user_space)
        };

        loop {
            // The execute method returns when system
            // calls or CPU exceptions occur.
            let user_event = user_mode.execute();
            // The CPU registers of the user space
            // can be accessed and manipulated via
            // the `UserContext` abstraction.
            let user_context = user_mode.context_mut();
            if UserEvent::Syscall == user_event {
                syscall::handle_syscall(user_context, current.user_space().unwrap());
            }
        }
    }

    // Kernel tasks are managed by the Framework,
    // while scheduling algorithms for them can be
    // determined by the users of the Framework.
    TaskOptions::new(user_task)
        .user_space(Some(user_space))
        .data(0)
        .build()
        .unwrap()
}


// use c style to fork sel4
// struct ApicBaseMsr(u32);

// impl ApicBaseMsr {
//     fn base_addr(&self) -> u32 {
//         todo!()
//     }
//     fn enabled(&self) -> u32 {
//         todo!()
//     }
//     fn x2apic(&self) -> u32 {
//         todo!()
//     }
// }

// struct ApicIcr1(u32);
// struct ApicIcr2(u32);
// struct ApicLvt(u32);
// struct ApicSvr(u32);
// struct ApicVersion(u32);
// struct Cpuid001hEax(u32);
// struct Cpuid001hEbx(u32);
// struct Cpuid007hEbx(u32);
// struct Cpuid007hEdx(u32);
// struct Cr3(u32);
// struct EndPoint([u64; 2]);
// struct GdtTss([u64; 2]);
// struct IA32ArchCapabilitiesMsr(u32);

// // bitflags! {
// //     /// MdbNodeNext conatins the next node in the list
// //     /// 1 bit indicate if the node is first badged or not
// //     /// 1 bit indicate if the node is revocable or not
// //     #[derive(Debug)]
// //     pub struct MdbNodeNext:u64 {
// //         const FistBadged = 1;
// //         const Revocable = 1 << 1;
// //         const _ = !0;
// //     }
// // }

// struct Notification([u64; 4]);
// struct Pml4e(u64);
// struct Pte(u64);
// struct TaskGate([u64; 2]);
// struct ThreadState([u64; 3]);
// struct Tss([u64; 3]);
// struct VmAttributes(u64);
// struct X2ApicIcr1(u32);
// struct X2ApicIcr2(u32);
// struct X86PatMsr([u32; 2]);
// struct AsidMap(u64);
// // #[derive(Debug)]
// // enum xxx {
// //     None = 0,
// //     VSpace = 1,
// // }

// struct GdtEntry(u64);
// enum GdtEntryTag {
//     Null,
//     Data,
//     Code,
// }
// struct IdtEntry([u64; 2]);
// enum IdtEntryTag {}
// struct LookupFault([u64; 2]);
// enum LookupFaultTag {}
// struct Pde(u64);
// enum PdeTag {}

// struct PdPte(u64);

// struct seL4_Fault {}

// struct X86IrqState {}

// /* arch independent object types */
// #[repr(u64)]
// enum EndPointState {
//     Idle = 0,
//     Send = 1,
//     Recv = 2,
// }
// #[repr(u64)]
// enum NotificationState {
//     Idle = 0,
//     Waiting = 1,
//     Active = 2,
// }
// /*
// We would like the actual 'tcb' region (the portion that contains the tcb_t) of the tcb
// to be as large as possible, but it still needs to be aligned. As the TCB object contains
// two sub objects the largest we can make either sub object whilst preserving size alignment
// is half the total size. To halve an object size defined in bits we just subtract 1

// A diagram of a TCB kernel object that is created from untyped:
//  _______________________________________
// |     |             |                   |
// |     |             |                   |
// |cte_t|   unused    |       tcb_t       |
// |     |(debug_tcb_t)|                   |
// |_____|_____________|___________________|
// 0     a             b                   c
// a = tcbCNodeEntries * sizeof(cte_t)
// b = BIT(TCB_SIZE_BITS)
// c = BIT(seL4_TCBBits)
// */

// /// _thread_state
// enum ThreadExecState {
//     Inactive,
//     Running,
//     Restart,
//     BlockedOnReceive,
//     BlockedOnSend,
//     BlockedOnReply,
//     BlockedOnNotification,
//     // TODO add feature
//     RunningVM,
//     IdleThreadState,
// }


// struct DeriveCapRet {
//     status: Exception,
//     cap: Cap,
// }

// struct FinaliseCapRet {
//     remainder: Cap,
//     cleanup_info: Cap,
// }

// mod tcb {
//     use super::Exception;
//     /// thread context block
//     pub struct TCB {}
//     struct TCBQueue {
//         head: *mut TCB,
//         end: *mut TCB,
//     }
//     fn tcbSchedEnqueue(tcb: *mut TCB) {}
//     fn tcbSchedAppend(tcb: *mut TCB) {}
//     fn tcbSchedDequeue(tcb: *mut TCB) {}

//     // Assuming the Rust equivalents of the C types are defined elsewhere
//     // e.g., `tcb_t`, `tcb_queue_t`, `bool_t`

//     // Function to append a TCB to a queue
//     pub fn tcbEPAppend(tcb: &TCB, queue: TCBQueue) -> TCBQueue {
//         // Implementation of the function
//         // ...

//         // Return the updated queue
//         queue
//     }

//     // Function to dequeue a TCB from a queue
//     pub fn tcbEPDequeue(tcb: &TCB, queue: TCBQueue) -> TCBQueue {
//         // Implementation of the function
//         // ...

//         // Return the updated queue
//         queue
//     }

//     // Function to set up a caller's capability
//     pub fn setupCallerCap(sender: &TCB, receiver: &TCB, canGrant: bool) {
//         // Implementation of the function
//         // ...
//     }

//     // Function to delete a caller's capability
//     pub fn deleteCallerCap(receiver: &TCB) {
//         // Implementation of the function
//         // ...
//     }

//     fn invokeTCB_Suspend(thread: *const TCB) -> Exception {
//         Exception::None
//     }
//     fn invokeTCB_Resume(thread: *const TCB) -> Exception {
//         Exception::None
//     }

//     // 根据配置决定是否定义特定的函数
//     #[cfg(feature = "CONFIG_KERNEL_MCS")]
//     fn invokeTCB_ThreadControlCaps(
//         target: *const TCB,
//         slot: *const CTE,
//         fh_newCap: Cap,
//         fh_srcSlot: *const CTE,
//         th_newCap: Cap,
//         th_srcSlot: *const CTE,
//         cRoot_newCap: Cap,
//         cRoot_srcSlot: *const CTE,
//         vRoot_newCap: Cap,
//         vRoot_srcSlot: *const CTE,
//         bufferAddr: Word,
//         bufferCap: Cap,
//         bufferSrcSlot: *const CTE,
//         updateFlags: thread_control_flag_t,
//     ) -> Exception;

//     #[cfg(feature = "CONFIG_KERNEL_MCS")]
//     fn invokeTCB_ThreadControlSched(
//         target: *const TCB,
//         slot: *const CTE,
//         fh_newCap: Cap,
//         fh_srcSlot: *const CTE,
//         mcp: prio_t,
//         priority: prio_t,
//         sc: *const sched_context_t,
//         updateFlags: thread_control_flag_t,
//     ) -> Exception;

//     // #[cfg(not(feature = "CONFIG_KERNEL_MCS"))]
//     // fn invokeTCB_ThreadControl(
//     //     target: *const TCB,
//     //     slot: *const CTE,
//     //     faultep: cptr_t,
//     //     mcp: prio_t,
//     //     priority: prio_t,
//     //     cRoot_newCap: Cap,
//     //     cRoot_srcSlot: *const CTE,
//     //     vRoot_newCap: Cap,
//     //     vRoot_srcSlot: *const CTE,
//     //     bufferAddr: Word,
//     //     bufferCap: Cap,
//     //     bufferSrcSlot: *const CTE,
//     //     updateFlags: thread_control_flag_t,
//     // ) -> Exception {
//     //     Exception::None
//     // }

//     fn invokeTCB_CopyRegisters(
//         dest: *const TCB,
//         src: *const TCB,
//         suspendSource: bool,
//         resumeTarget: bool,
//         transferFrame: bool,
//         transferInteger: bool,
//         transferArch: usize,
//     ) -> Exception {
//         todo!()
//     }

//     // Assuming the Rust equivalents of the C types are defined elsewhere
//     // e.g., `TCB`, `notification_t`, `word_t`, `cptr_t`, `seL4_MessageInfo_t`, `exception_t`

//     // invokeTCB_ReadRegisters function
//     pub fn invokeTCB_ReadRegisters(
//         src: &TCB,
//         suspendSource: bool,
//         n: usize,
//         arch: usize,
//         call: bool,
//     ) -> Exception {
//         // Implementation of the function
//         // ...
//         Exception::None // Placeholder return value
//     }

//     // invokeTCB_WriteRegisters function
//     pub fn invokeTCB_WriteRegisters(
//         dest: &TCB,
//         resumeTarget: bool,
//         n: usize,
//         arch: usize,
//         buffer: &[usize],
//     ) -> Exception {
//         // Implementation of the function
//         // ...
//         Exception::None // Placeholder return value
//     }

//     // invokeTCB_NotificationControl function
//     // pub fn invokeTCB_NotificationControl(tcb: &TCB, ntfnPtr: &notification_t) -> Exception {
//     //     // Implementation of the function
//     //     // ...
//     //     Exception::None // Placeholder return value
//     // }

//     // getExtraCPtr function
//     // pub fn getExtraCPtr(bufferPtr: &[word_t], i: word_t) -> cptr_t {
//     //     // Implementation of the function
//     //     // ...
//     //     0 as cptr_t // Placeholder return value
//     // }

//     // setExtraBadge function
//     // pub fn setExtraBadge(bufferPtr: &mut [word_t], badge: word_t, i: word_t) {
//     //     // Implementation of the function
//     //     // ...
//     // }

//     // lookupExtraCaps function
//     // pub fn lookupExtraCaps(
//     //     thread: &TCB,
//     //     bufferPtr: &mut [word_t],
//     //     info: seL4_MessageInfo_t,
//     // ) -> exception_t {
//     //     // Implementation of the function
//     //     // ...
//     //     exception_t::None // Placeholder return value
//     // }

//     // setMRs_syscall_error function
//     // pub fn setMRs_syscall_error(thread: &TCB, receiveIPCBuffer: &mut [word_t]) -> word_t {
//     //     // Implementation of the function
//     //     // ...
//     //     0 as word_t // Placeholder return value
//     // }

//     // Arch_decodeTransfer function
//     // pub const fn Arch_decodeTransfer(flags: word_t) -> word_t {
//     //     // Implementation of the function
//     //     // ...
//     //     0 as word_t // Placeholder return value
//     // }

//     // Arch_performTransfer function
//     // pub const fn Arch_performTransfer(
//     //     arch: word_t,
//     //     tcb_src: &TCB,
//     //     tcb_dest: &TCB,
//     // ) -> exception_t {
//     //     // Implementation of the function
//     //     // ...
//     //     exception_t::None // Placeholder return value
//     // }

//     // setThreadName function (only included if CONFIG_DEBUG_BUILD is defined)
//     #[cfg(feature = "debug_build")]
//     pub fn setThreadName(thread: &TCB, name: &str) {
//         // Implementation of the function
//         // ...
//     }
// }

// mod untyped {
//     use super::Exception;
//     // use generic
//     fn decode_untyped_invocation(retype_base: usize) -> Exception {
//         todo!()
//     }

//     fn invoke_untyped_retype() -> Exception {
//         todo!()
//     }
// }
