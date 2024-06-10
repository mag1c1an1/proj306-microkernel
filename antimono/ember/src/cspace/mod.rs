use alloc::boxed::Box;
use core::intrinsics::atomic_singlethreadfence_acqrel;
use core::ptr::slice_from_raw_parts_mut;

use aster_frame::vm::{HasPaddr, VmFrameVec};

use capability::Capability;

use crate::{EmberResult, max_free_index};
use crate::common::MemRef;
use crate::common::region::paddr_to_kaddr;
use crate::cspace::raw::{CapType, RawCap};
use crate::cspace::raw::cte::CTE;
use crate::cspace::raw::mdb::MdbNode;

pub mod capability;
pub mod raw;

#[repr(transparent)]
#[derive(Debug, Default)]
pub struct Slot(CTE);

impl Slot {
    /// is this slot empty (contain a null capability)?
    pub fn is_empty(&self) -> bool {
        self.0.raw_cap.raw_typ() == (CapType::Null as usize) && self.0.mdb_node.prev() == 0 && self.0.mdb_node.next() == 0
    }

    pub fn raw_cap(&self) -> &RawCap {
        &self.0.raw_cap
    }

    pub fn mdb_node_mut(&mut self) -> &mut MdbNode {
        &mut self.0.mdb_node
    }

    pub fn set_next(&mut self) {
        todo!()
    }

    pub fn next(&self) -> &Slot {
        todo!()
    }

    pub fn next_mut(&self) -> &mut Slot {
        todo!()
    }
    pub fn set_prev(&mut self) {}
    pub fn prev(&self) -> &mut Slot {
        todo!()
    }
    pub fn prev_mut(&self) -> &mut Slot {
        todo!()
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

    /// If creating a child UntypedCap, don't allow new objects to be created in the
    ///  parent.
    pub fn set_untyped_as_full(&mut self, other_cap: &RawCap) {
        if self.0.raw_cap.cap_type() == CapType::Untyped
            && other_cap.cap_type() == CapType::Untyped
            && self.0.raw_cap.untyped_ptr() == other_cap.untyped_ptr()
            && self.0.raw_cap.untyped_block_size() == other_cap.untyped_block_size()
        {
            self.0.raw_cap.set_untyped_free_index(
                max_free_index!(self.0.raw_cap.untyped_block_size())
            );
        }
    }


    /// cteInsert in sel4
    pub fn insert(&mut self, src_slot: &mut Slot, new_cap: RawCap) {
        assert!(self.is_empty());
        let new_is_revocable = new_cap.is_revocable(src_slot.raw_cap()) as usize;
        self.0.raw_cap = new_cap;
        self.0.mdb_node = MdbNode::new(
            src_slot as *const Slot as usize,
            0,
            new_is_revocable,
            new_is_revocable,
        );
        src_slot.mdb_node_mut().set_next(self as *const Slot as usize);
    }
}


/// slot's memory is in the inner
#[derive(Debug)]
pub struct CNodeObject
{
    /// slots' residence
    /// may bigger than slots
    mem: VmFrameVec,
    slots: MemRef<[Slot]>,
}

impl CNodeObject {
    /// len : slots len
    pub unsafe fn try_new(mem: VmFrameVec, len: usize) -> EmberResult<Self> {
        let start = paddr_to_kaddr(mem.0[0].paddr());
        let slice_ptr = slice_from_raw_parts_mut(start as *mut Slot, len);
        let mut slots = Box::from_raw(slice_ptr);
        // init
        for slot in slots.iter_mut() {
            *slot = Slot::default();
        }

        Ok(
            Self {
                mem,
                slots: MemRef::new(slots),
            }
        )
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
    /// cte insert in sel4
    pub fn insert(&mut self, index: usize, src: &Slot, cap: RawCap) {
        self.slots[index].insert(src, cap)
    }
    pub fn insert_new_cap() { todo!() }
    /// move
    pub fn mov() {}
    pub fn swap() {}
}


impl AsRef<[Slot]> for CNodeObject {
    fn as_ref(&self) -> &[Slot] {
        self.slots.as_ref()
    }
}

impl AsMut<[Slot]> for CNodeObject {
    fn as_mut(&mut self) -> &mut [Slot] {
        self.slots.as_mut()
    }
}


unsafe impl Send for CNodeObject {}

unsafe impl Sync for CNodeObject {}


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
