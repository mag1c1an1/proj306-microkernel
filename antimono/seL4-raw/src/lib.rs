#![no_std]
#![allow(clippy::upper_case_acronyms)]

use bitfield::bitfield;
use bitflags::bitflags;
// use bitflags::bitflags;
use core::fmt::Debug;

/// use c style to fork sel4
struct ApicBaseMsr(u32);

impl ApicBaseMsr {
    fn base_addr(&self) -> u32 {
        todo!()
    }
    fn enabled(&self) -> u32 {
        todo!()
    }
    fn x2apic(&self) -> u32 {
        todo!()
    }
}

struct ApicIcr1(u32);
struct ApicIcr2(u32);
struct ApicLvt(u32);
struct ApicSvr(u32);
struct ApicVersion(u32);
struct Cpuid001hEax(u32);
struct Cpuid001hEbx(u32);
struct Cpuid007hEbx(u32);
struct Cpuid007hEdx(u32);
struct Cr3(u32);
struct EndPoint([u64; 2]);
struct GdtTss([u64; 2]);
struct IA32ArchCapabilitiesMsr(u32);
pub struct MdbNode([u64; 2]);

// bitflags! {
//     /// MdbNodeNext conatins the next node in the list
//     /// 1 bit indicate if the node is first badged or not
//     /// 1 bit indicate if the node is revocable or not
//     #[derive(Debug)]
//     pub struct MdbNodeNext:u64 {
//         const FistBadged = 1;
//         const Revocable = 1 << 1;
//         const _ = !0;
//     }
// }

bitflags! {
    #[derive(Debug)]
    pub struct MdbFlags:u64 {
        const FistBadged = 1;
        const Revocable = 1 << 1;
    }
}

bitfield! {
    pub struct MdbNodeNext(u64);
    impl Debug;
    pub first_badged,set_first_badged: 0,0;
    pub revocable,set_revocable: 1,1;
    pub next,set_next: 63,2;
}

impl MdbNode {
    /// TODO Refactor this
    pub fn new(next: u64, revocable: u64, first_badged: u64, prev: u64) -> MdbNode {
        let f = MdbFlags::from_bits_truncate(revocable);
        todo!()
    }
    pub fn next(&self) -> MdbNode {
        todo!()
    }
    pub fn set_next(&mut self) {
        todo!()
    }
    pub fn prev(&self) -> MdbNode {
        todo!()
    }
    pub fn set_prev(&mut self) {
        todo!()
    }
    pub fn revocable(&self) -> MdbNodeNext {
        todo!()
    }
    pub fn set_revocable(&mut self) {
        todo!()
    }
    pub fn first_badged(&self) -> MdbNodeNext {
        todo!()
    }
    pub fn set_first_badged(&mut self) {
        todo!()
    }
}

struct Notification([u64; 4]);
struct Pml4e(u64);
struct Pte(u64);
struct TaskGate([u64; 2]);
struct ThreadState([u64; 3]);
struct Tss([u64; 3]);
struct VmAttributes(u64);
struct X2ApicIcr1(u32);
struct X2ApicIcr2(u32);
struct X86PatMsr([u32; 2]);
struct AsidMap(u64);
// #[derive(Debug)]
// enum xxx {
//     None = 0,
//     VSpace = 1,
// }

bitfield! {
    pub struct CPtr(u64);
    impl Debug;
    pub tag,set_tag: 63,59;
    pub ptr,set_ptr: 58,0;
}

bitfield! {
    pub struct UntypedRights(u64);
    impl Debug;
    pub free_index, set_free_index: 63,17;
    pub device, set_device: 7;
    pub block_size,set_block_size: 2,0;
}

struct Cap([u64; 2]);
enum CapTag {
    Null = 0,
    Untyped = 2,
    Endpoint = 4,
    Notification = 6,
    Reply = 8,
    Cnode = 10,
    Thread = 12,
    IrqControl = 14,
    IrqHandler = 16,
    Zombie = 18,
    Domain = 20,
    Frame = 1,
    PageTable = 3,
    PageDirectory = 5,
    PDPT = 7,
    PML4 = 9,
    AsidControl = 11,
    AsidPool = 13,
    IOPort = 19,
    IOPortControl = 31,
}
struct GdtEntry(u64);
enum GdtEntryTag {
    Null,
    Data,
    Code,
}
struct IdtEntry([u64; 2]);
enum IdtEntryTag {}
struct LookupFault([u64; 2]);
enum LookupFaultTag {}
struct Pde(u64);
enum PdeTag {}

struct PdPte(u64);

struct seL4_Fault {}

struct X86IrqState {}

/// table entry
pub struct CTE {
    cap: Cap,
    cte_mdb_node: MdbNode,
}

/* arch independent object types */
#[repr(u64)]
enum EndPointState {
    Idle = 0,
    Send = 1,
    Recv = 2,
}
#[repr(u64)]
enum NotificationState {
    Idle = 0,
    Waiting = 1,
    Active = 2,
}
/*
We would like the actual 'tcb' region (the portion that contains the tcb_t) of the tcb
to be as large as possible, but it still needs to be aligned. As the TCB object contains
two sub objects the largest we can make either sub object whilst preserving size alignment
is half the total size. To halve an object size defined in bits we just subtract 1

A diagram of a TCB kernel object that is created from untyped:
 _______________________________________
|     |             |                   |
|     |             |                   |
|cte_t|   unused    |       tcb_t       |
|     |(debug_tcb_t)|                   |
|_____|_____________|___________________|
0     a             b                   c
a = tcbCNodeEntries * sizeof(cte_t)
b = BIT(TCB_SIZE_BITS)
c = BIT(seL4_TCBBits)
*/

/// _thread_state
enum ThreadExecState {
    Inactive,
    Running,
    Restart,
    BlockedOnReceive,
    BlockedOnSend,
    BlockedOnReply,
    BlockedOnNotification,
    // TODO add feature
    RunningVM,
    IdleThreadState,
}

mod error {
    #[repr(u32)]
    pub enum Exception {
        None = 0,
        Fault = 1,
        Lookup,
        SysCall,
        Preempted,
    }

    type Result<T, E = Exception> = core::result::Result<T, E>;
}

pub use error::*;

/// C style
impl CTE {
    fn insert(new_cap: Cap, src_slot: *mut CTE, dest_slot: *mut CTE) {
        todo!()
    }

    fn mov(new_cap: Cap, src_slot: *mut CTE, dest_slot: *mut CTE) {
        todo!()
    }
    fn swap_for_delete() {}
    fn swap() {}
    fn revoke() {}
    fn delete() {}
}

impl CTE {
    fn empty_slot(&mut self) {}
}

struct DeriveCapRet {
    status: Exception,
    cap: Cap,
}

struct FinaliseCapRet {
    remainder: Cap,
    cleanup_info: Cap,
}

mod cnode {
    use core::ptr;

    use crate::{tcb::TCB, Cap, Exception, CTE};

    fn decode_cnode_invocation() {}
    // invokeCNodeRevoke函数的Rust版本
    fn invoke_cnode_revoke(destSlot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // invokeCNodeDelete函数的Rust版本
    fn invokeCNodeDelete(destSlot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // invokeCNodeCancelBadgedSends函数的Rust版本
    fn invokeCNodeCancelBadgedSends(cap: Cap) -> Exception {
        // 函数实现
        Exception::None
    }

    // invokeCNodeInsert函数的Rust版本
    fn invokeCNodeInsert(cap: Cap, srcSlot: &mut CTE, destSlot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // invokeCNodeMove函数的Rust版本
    fn invokeCNodeMove(cap: Cap, srcSlot: &mut CTE, destSlot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // invokeCNodeRotate函数的Rust版本
    fn invokeCNodeRotate(
        cap1: Cap,
        cap2: Cap,
        slot1: &mut CTE,
        slot2: &mut CTE,
        slot3: &mut CTE,
    ) -> Exception {
        // 函数实现
        Exception::None
    }

    // cteInsert函数的Rust版本
    fn cteInsert(newCap: Cap, srcSlot: &mut CTE, destSlot: &mut CTE) {
        // 函数实现
    }

    // cteMove函数的Rust版本
    fn cteMove(newCap: Cap, srcSlot: &mut CTE, destSlot: &mut CTE) {
        // 函数实现
    }

    // capSwapForDelete函数的Rust版本
    fn capSwapForDelete(slot1: &mut CTE, slot2: &mut CTE) {
        // 函数实现
    }

    // cteSwap函数的Rust版本
    fn cteSwap(cap1: Cap, slot1: &mut CTE, cap2: Cap, slot2: &mut CTE) {
        // 函数实现
    }

    // cteRevoke函数的Rust版本
    fn cteRevoke(slot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // cteDelete函数的Rust版本
    fn cteDelete(slot: &mut CTE, exposed: bool) -> Exception {
        // 函数实现
        Exception::None
    }

    // cteDeleteOne函数的Rust版本
    fn cteDeleteOne(slot: &mut CTE) {
        // 函数实现
    }

    // insertNewCap函数的Rust版本
    fn insertNewCap(parent: &mut CTE, slot: &mut CTE, cap: Cap) {
        // 函数实现
    }

    // isMDBParentOf函数的Rust版本
    fn isMDBParentOf(cte_a: &CTE, cte_b: &CTE) -> bool {
        // 函数实现
        false
    }

    // ensureNoChildren函数的Rust版本
    fn ensureNoChildren(slot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // ensureEmptySlot函数的Rust版本
    fn ensureEmptySlot(slot: &mut CTE) -> Exception {
        // 函数实现
        Exception::None
    }

    // isFinalCapability函数的Rust版本
    fn isFinalCapability(cte: &CTE) -> bool {
        // 函数实现
        false
    }

    // slotCapLongRunningDelete函数的Rust版本
    fn slotCapLongRunningDelete(slot: &CTE) -> bool {
        // 函数实现
        false
    }

    // getReceiveSlots函数的Rust版本
    fn getReceiveSlots(thread: &TCB, buffer: &mut u64) -> *mut CTE {
        // 函数实现
        ptr::null_mut::<CTE>()
    }
}

mod tcb {
    use super::Exception;
    /// thread context block
    pub struct TCB {}
    struct TCBQueue {
        head: *mut TCB,
        end: *mut TCB,
    }
    fn tcbSchedEnqueue(tcb: *mut TCB) {}
    fn tcbSchedAppend(tcb: *mut TCB) {}
    fn tcbSchedDequeue(tcb: *mut TCB) {}

    // Assuming the Rust equivalents of the C types are defined elsewhere
    // e.g., `tcb_t`, `tcb_queue_t`, `bool_t`

    // Function to append a TCB to a queue
    pub fn tcbEPAppend(tcb: &TCB, queue: TCBQueue) -> TCBQueue {
        // Implementation of the function
        // ...

        // Return the updated queue
        queue
    }

    // Function to dequeue a TCB from a queue
    pub fn tcbEPDequeue(tcb: &TCB, queue: TCBQueue) -> TCBQueue {
        // Implementation of the function
        // ...

        // Return the updated queue
        queue
    }

    // Function to set up a caller's capability
    pub fn setupCallerCap(sender: &TCB, receiver: &TCB, canGrant: bool) {
        // Implementation of the function
        // ...
    }

    // Function to delete a caller's capability
    pub fn deleteCallerCap(receiver: &TCB) {
        // Implementation of the function
        // ...
    }

    fn invokeTCB_Suspend(thread: *const TCB) -> Exception {
        Exception::None
    }
    fn invokeTCB_Resume(thread: *const TCB) -> Exception {
        Exception::None
    }

    // 根据配置决定是否定义特定的函数
    #[cfg(feature = "CONFIG_KERNEL_MCS")]
    fn invokeTCB_ThreadControlCaps(
        target: *const TCB,
        slot: *const CTE,
        fh_newCap: Cap,
        fh_srcSlot: *const CTE,
        th_newCap: Cap,
        th_srcSlot: *const CTE,
        cRoot_newCap: Cap,
        cRoot_srcSlot: *const CTE,
        vRoot_newCap: Cap,
        vRoot_srcSlot: *const CTE,
        bufferAddr: Word,
        bufferCap: Cap,
        bufferSrcSlot: *const CTE,
        updateFlags: thread_control_flag_t,
    ) -> Exception;

    #[cfg(feature = "CONFIG_KERNEL_MCS")]
    fn invokeTCB_ThreadControlSched(
        target: *const TCB,
        slot: *const CTE,
        fh_newCap: Cap,
        fh_srcSlot: *const CTE,
        mcp: prio_t,
        priority: prio_t,
        sc: *const sched_context_t,
        updateFlags: thread_control_flag_t,
    ) -> Exception;

    // #[cfg(not(feature = "CONFIG_KERNEL_MCS"))]
    // fn invokeTCB_ThreadControl(
    //     target: *const TCB,
    //     slot: *const CTE,
    //     faultep: cptr_t,
    //     mcp: prio_t,
    //     priority: prio_t,
    //     cRoot_newCap: Cap,
    //     cRoot_srcSlot: *const CTE,
    //     vRoot_newCap: Cap,
    //     vRoot_srcSlot: *const CTE,
    //     bufferAddr: Word,
    //     bufferCap: Cap,
    //     bufferSrcSlot: *const CTE,
    //     updateFlags: thread_control_flag_t,
    // ) -> Exception {
    //     Exception::None
    // }

    fn invokeTCB_CopyRegisters(
        dest: *const TCB,
        src: *const TCB,
        suspendSource: bool,
        resumeTarget: bool,
        transferFrame: bool,
        transferInteger: bool,
        transferArch: usize,
    ) -> Exception {
        todo!()
    }

    // Assuming the Rust equivalents of the C types are defined elsewhere
    // e.g., `TCB`, `notification_t`, `word_t`, `cptr_t`, `seL4_MessageInfo_t`, `exception_t`

    // invokeTCB_ReadRegisters function
    pub fn invokeTCB_ReadRegisters(
        src: &TCB,
        suspendSource: bool,
        n: usize,
        arch: usize,
        call: bool,
    ) -> Exception {
        // Implementation of the function
        // ...
        Exception::None // Placeholder return value
    }

    // invokeTCB_WriteRegisters function
    pub fn invokeTCB_WriteRegisters(
        dest: &TCB,
        resumeTarget: bool,
        n: usize,
        arch: usize,
        buffer: &[usize],
    ) -> Exception {
        // Implementation of the function
        // ...
        Exception::None // Placeholder return value
    }

    // invokeTCB_NotificationControl function
    // pub fn invokeTCB_NotificationControl(tcb: &TCB, ntfnPtr: &notification_t) -> Exception {
    //     // Implementation of the function
    //     // ...
    //     Exception::None // Placeholder return value
    // }

    // getExtraCPtr function
    // pub fn getExtraCPtr(bufferPtr: &[word_t], i: word_t) -> cptr_t {
    //     // Implementation of the function
    //     // ...
    //     0 as cptr_t // Placeholder return value
    // }

    // setExtraBadge function
    // pub fn setExtraBadge(bufferPtr: &mut [word_t], badge: word_t, i: word_t) {
    //     // Implementation of the function
    //     // ...
    // }

    // lookupExtraCaps function
    // pub fn lookupExtraCaps(
    //     thread: &TCB,
    //     bufferPtr: &mut [word_t],
    //     info: seL4_MessageInfo_t,
    // ) -> exception_t {
    //     // Implementation of the function
    //     // ...
    //     exception_t::None // Placeholder return value
    // }

    // setMRs_syscall_error function
    // pub fn setMRs_syscall_error(thread: &TCB, receiveIPCBuffer: &mut [word_t]) -> word_t {
    //     // Implementation of the function
    //     // ...
    //     0 as word_t // Placeholder return value
    // }

    // Arch_decodeTransfer function
    // pub const fn Arch_decodeTransfer(flags: word_t) -> word_t {
    //     // Implementation of the function
    //     // ...
    //     0 as word_t // Placeholder return value
    // }

    // Arch_performTransfer function
    // pub const fn Arch_performTransfer(
    //     arch: word_t,
    //     tcb_src: &TCB,
    //     tcb_dest: &TCB,
    // ) -> exception_t {
    //     // Implementation of the function
    //     // ...
    //     exception_t::None // Placeholder return value
    // }

    // setThreadName function (only included if CONFIG_DEBUG_BUILD is defined)
    #[cfg(feature = "debug_build")]
    pub fn setThreadName(thread: &TCB, name: &str) {
        // Implementation of the function
        // ...
    }
}

mod untyped {
    use super::Exception;
    // use generic
    fn decode_untyped_invocation(retype_base: usize) -> Exception {
        todo!()
    }

    fn invoke_untyped_retype() -> Exception {
        todo!()
    }
}
