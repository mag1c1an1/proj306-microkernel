use core::ptr::NonNull;

use capability::Capability;

use crate::cspace::raw::{CapType, RawCap};
use crate::cspace::raw::cte::CTE;
use crate::EmberResult;

mod capability;
mod raw;

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
}


/// care about drop?
#[derive(Debug)]
pub struct CNode
{
    start: NonNull<[Slot]>,
    _phantom: core::marker::PhantomData<[Slot]>,
}

impl CNode {
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
        unsafe { self.start.as_ref() }
    }
}

impl AsMut<[Slot]> for CNode {
    fn as_mut(&mut self) -> &mut [Slot] {
        unsafe {
            self.start.as_mut()
        }
    }
}


unsafe impl Send for CNode {}

unsafe impl Sync for CNode {}


mod test {
    use core::mem::size_of;

    use ktest::ktest;

    use crate::bit;
    use crate::cspace::Slot;
    use crate::sel4::cnode::ROOT_CNODE_SIZE_BITS;
    use crate::sel4::sys::seL4_SlotBits;


    #[ktest]
    fn slot_size_test() {
        assert_eq!(bit!(seL4_SlotBits), size_of::<Slot>());
        assert_eq!(bit!(seL4_SlotBits as usize + ROOT_CNODE_SIZE_BITS), size_of::<[Slot; bit!(ROOT_CNODE_SIZE_BITS)]>())
    }
}
