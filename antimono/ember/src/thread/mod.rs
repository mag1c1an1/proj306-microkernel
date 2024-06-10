use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::{Arc, Weak};
use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};

use aster_frame::sync::{Mutex, MutexGuard};
use aster_frame::task::Task;
use aster_frame::vm::{HasPaddr, VmFrameVec, VmSpace};

use crate::{bit, EmberResult};
use crate::common::MemRef;
use crate::common::region::paddr_to_kaddr;
use crate::cspace::{CNodeObject, Slot};
use crate::sel4::tcb::{TCB_CNODE_ENTRIES, TCB_SIZE_BITS};
use crate::thread::raw::{DebugRawTcb, RawTcb};
use crate::thread::state::{AtomicThreadState, ThreadState};

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
    atomic_thread_state: AtomicThreadState,
}

impl Thread {
    pub fn new(task: Arc<Task>, tcb_object: TcbObject, vm_space: Arc<VmSpace>, state: ThreadState) -> Self {
        Thread {
            tid: allocate_tid(),
            name: String::new(),
            task,
            vm_space,
            tcb_object: Mutex::new(tcb_object),
            atomic_thread_state: AtomicThreadState::new(state),
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
    pub fn tcb_object(&self) -> MutexGuard<TcbObject> {
        self.tcb_object.lock()
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
    mem: VmFrameVec,
    //
    tcb_cnode: CNodeObject,
    debug_raw_tcb: MemRef<DebugRawTcb>,
    tcb: MemRef<RawTcb>,
}


impl TcbObject {
    pub unsafe fn try_new(mem: VmFrameVec, raw_tcb: RawTcb) -> EmberResult<Self> {
        let start = paddr_to_kaddr(mem.0[0].paddr());
        let tcb_cnode = CNodeObject::try_new(mem.clone(), TCB_CNODE_ENTRIES)?;
        // TCB_CNODE_SIZE_BITS
        let tcb_cnode_size = TCB_CNODE_ENTRIES * size_of::<Slot>();
        debug_assert!(tcb_cnode_size == 160, "wrong size");
        let debug_tcb_size = bit!(TCB_SIZE_BITS) - tcb_cnode_size;
        debug_assert!(debug_tcb_size == 864, "wrong size");
        let debug_raw_tcb = MemRef::new(Box::from_raw((start + tcb_cnode_size) as *mut DebugRawTcb));
        let tcb = MemRef::new(Box::from_raw((start + bit!(TCB_SIZE_BITS)) as *mut RawTcb)));
        Ok(
            Self {
                mem,
                tcb_cnode,
                debug_raw_tcb,
                tcb,
            }
        )
    }
    pub fn raw_tcb_mut(&mut self) -> &mut RawTcb {
        self.tcb.as_mut()
    }


    pub fn tcb_cnode_mut(&mut self) -> &mut CNodeObject {
        &mut self.tcb_cnode
    }
}