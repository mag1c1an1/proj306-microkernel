use alloc::string::{String, ToString};
use alloc::sync::{Arc, Weak};
use core::mem::size_of;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU32, Ordering};

use aster_frame::sync::Mutex;
use aster_frame::task::Task;
use aster_frame::vm::VmSpace;

use crate::bit;
use crate::cspace::{CNode, Slot};
use crate::sel4::tcb::{TCB_CNODE_ENTRIES, TCB_SIZE_BITS};
use crate::thread::raw::{DebugRawTcb, RawTcb};

pub mod task;
pub mod exception;
pub mod state;
pub mod raw;

pub type Tid = u32;

static TID_ALLOCATOR: AtomicU32 = AtomicU32::new(0);

pub fn allocate_tid() -> Tid {
    TID_ALLOCATOR.fetch_add(1, Ordering::SeqCst)
}

pub struct Thread {
    tid: Tid,
    name: String,
    task: Arc<Task>,
    vm_space: Arc<VmSpace>,
    tcb_object: Mutex<TcbObject>,
}

impl Thread {
    pub fn new(task: Arc<Task>, tcb_object: TcbObject, vm_space: Arc<VmSpace>) -> Self {
        Thread {
            tid: allocate_tid(),
            name: String::new(),
            task,
            vm_space,
            tcb_object: Mutex::new(tcb_object),
        }
    }
    pub fn current() -> Arc<Thread> {
        let task = Task::current();
        let thread = task
            .data()
            .downcast_ref::<Weak<Thread>>()
            .expect("[Internal Error] task data should points to weak<thread>");
        thread
            .upgrade()
            .expect("[Internal Error] current thread cannot be None")
    }

    pub(in crate::thread) fn task(&self) -> &Arc<Task> {
        &self.task
    }
    pub fn tid(&self) -> Tid {
        self.tid
    }
    /// todo
    pub fn name(&self) -> String {
        "".to_string()
    }
    pub fn set_name(&mut self, name: String) {}
    pub fn state(&self) {}
    pub fn run(&self) {
        self.task.run()
    }
    /// yield now
    pub fn suspend(&self) {
        Task::yield_now()
    }
    pub fn restart(&self) {
        todo!()
    }
    pub fn set_domain() {}
    pub fn set_priority() {}
    pub fn set_mc_priority() {}
    pub fn set_thread_state() {}
}

pub fn do_ipc_transfer() {}


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
#[derive(Debug)]
pub struct TcbObject
{
    inner: TcbObjectInner,
}

#[derive(Debug)]
struct TcbObjectInner {
    tcb_cnode: NonNull<CNode>,
    debug_tcb: NonNull<DebugRawTcb>,
    tcb: NonNull<RawTcb>,
}

unsafe impl Send for TcbObject {}

unsafe impl Sync for TcbObject {}


impl TcbObject {
    pub(crate) fn new(start: NonNull<u8>) -> Self {
        unsafe {
            let tcb_cnode = start.cast::<CNode>();
            let tcb_cnode_size = TCB_CNODE_ENTRIES * size_of::<Slot>();
            debug_assert!(tcb_cnode_size == 160, "wrong size");
            let debug_tcb_size = bit!(TCB_SIZE_BITS) - tcb_cnode_size;
            debug_assert!(debug_tcb_size == 864, "wrong size");
            let debug_tcb = start.offset(tcb_cnode_size as isize).cast::<DebugRawTcb>();
            let tcb = start.offset(bit!(TCB_SIZE_BITS) as isize).cast::<RawTcb>();
            Self {
                inner: TcbObjectInner {
                    tcb_cnode,
                    debug_tcb,
                    tcb,
                }
            }
        }
    }
}