use alloc::boxed::Box;
use alloc::sync::Arc;
use core::mem::ManuallyDrop;
use core::ptr::slice_from_raw_parts_mut;

use aster_frame::vm::{HasPaddr, VmFrameVec};

use capability::Capability;

use crate::common::region::paddr_to_kaddr;
use crate::cspace::raw::{CapType, RawCap};
use crate::cspace::raw::cte::CTE;
use crate::cspace::raw::mdb::MdbNode;
use crate::EmberResult;
use crate::sel4::cnode::ROOT_CNODE_SIZE_BITS;

pub mod capability;
pub mod raw;

#[repr(transparent)]
#[derive(Debug, Default)]
pub struct Slot(CTE);

impl Slot {
    /// is this slot empty (contain a null capability)?
    pub fn is_empty(&self) -> bool {
        self.0.raw_cap.typ() == (CapType::Null as usize)
    }
    pub fn derive(&mut self, cap: &RawCap) -> EmberResult<RawCap> {
        todo!()
    }
    pub fn arch_derive(&mut self, cap: &RawCap) -> EmberResult<RawCap> {
        todo!()
    }
    pub fn no_children(&self) -> bool {
        todo!()
    }
    pub fn is_parent_of(&self, child: &Self) -> bool {
        todo!()
    }
    pub fn is_final(&self) -> bool { todo!() }
    pub fn is_long_running_delete(&self) -> bool { todo!() }
    pub fn finalise(&mut self, immediate: bool) -> EmberResult<RawCap> { todo!() }
    pub fn delete_all(&mut self) -> EmberResult<()> { todo!() }
    pub fn delete(&mut self) {}
    pub fn set_empty(&mut self, cleanup_info: &RawCap) { todo!() }
    pub fn reduce_zombie(&mut self, immediate: bool) -> EmberResult<()> { todo!() }
    pub fn revoke(&mut self) -> EmberResult<()> { todo!() }
    pub fn write_slot(&mut self, cap: RawCap) {
        self.0.raw_cap = cap;
        // init a new MdbNode
        let mut mdb = MdbNode::default();
        mdb.set_first_badged(1);
        mdb.set_revocable(1);
        self.0.mdb_node = mdb;
    }
}


/// slot's memory is in the inner
#[derive(Debug)]
pub struct CNode
{
    mem: Arc<VmFrameVec>,
    slots: ManuallyDrop<Box<[Slot]>>,
}

impl CNode {
    pub unsafe fn new(mem: Arc<VmFrameVec>) -> Self {
        let start = paddr_to_kaddr(mem.0[0].paddr());
        let slice_ptr = slice_from_raw_parts_mut(start as *mut Slot, *ROOT_CNODE_SIZE_BITS);
        let slots = Box::from_raw(slice_ptr);
        Self {
            mem,
            slots: ManuallyDrop::new(slots),
        }
    }


    pub fn write_slot(&mut self, index: usize, cap: RawCap) {
        self.slots[index].write_slot(cap)
    }

    pub fn get_slot(&self, index: usize) -> &Slot {
        &self.as_ref()[index]
    }
    pub fn get_slot_mut(&mut self, index: usize) -> &mut Slot {
        &mut self.as_mut()[index]
    }
    pub fn insert() {
        todo!()
    }
    pub fn insert_new_cap() { todo!() }
    /// move
    pub fn mov() {}
    pub fn swap() {}
}


impl AsRef<[Slot]> for CNode {
    fn as_ref(&self) -> &[Slot] {
        self.slots.as_ref()
    }
}

impl AsMut<[Slot]> for CNode {
    fn as_mut(&mut self) -> &mut [Slot] {
        self.slots.as_mut()
    }
}


unsafe impl Send for CNode {}

unsafe impl Sync for CNode {}


mod test {
    use core::mem::size_of;

    use ktest::ktest;

    use crate::bit;
    use crate::cspace::Slot;
    use crate::sel4::sys::seL4_SlotBits;

    #[ktest]
    fn slot_size_test() {
        assert_eq!(bit!(seL4_SlotBits), size_of::<Slot>());
    }
}
