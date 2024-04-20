use core::ptr;

use crate::{
    error::{Exception, Result},
    structures::{
        cap::{Cap, CapTag},
        MdbNode,
    },
};

use super::{is_cap_revocable, untyped::max_free_index};

#[derive(Debug, Clone, Copy)]
/// cap table entry
/// Maybe use MaybeUninit
pub struct CTE {
    pub cap: Cap,
    pub mdb_node: MdbNode,
}

impl CTE {
    /// mdb_node's prev and next is NULL
    pub fn is_empty(&self) -> bool {
        let prev = self.mdb_node.prev() as usize;
        let next = self.mdb_node.next() as usize;
        prev == 0 && next == 0
    }

    // TODO
    pub fn is_mdb_parent_of(&self, other: &Self) -> bool {
        // same region as
        let cap = &self.cap;
        match CapTag::from(cap.typ()) {
            CapTag::Null => todo!(),
            CapTag::Untyped => todo!(),
            CapTag::Endpoint => todo!(),
            CapTag::Notification => todo!(),
            CapTag::Reply => todo!(),
            CapTag::Cnode => todo!(),
            CapTag::Thread => todo!(),
            CapTag::IrqControl => todo!(),
            CapTag::IrqHandler => todo!(),
            CapTag::Zombie => todo!(),
            CapTag::Domain => todo!(),
            CapTag::Frame => todo!(),
            CapTag::PageTable => todo!(),
            CapTag::PageDirectory => todo!(),
            CapTag::PDPT => todo!(),
            CapTag::PML4 => todo!(),
            CapTag::AsidControl => todo!(),
            CapTag::AsidPool => todo!(),
            CapTag::IOPort => todo!(),
            CapTag::IOPortControl => todo!(),
        }
    }

    pub fn insert(&mut self) {
        todo!()
    }

    pub fn swap(&mut self) {
        todo!()
    }

    pub fn mov(&mut self) {
        todo!()
    }

    pub fn swap_for_delete(&mut self) {
        todo!()
    }

    pub fn revoke(&mut self) -> Result<()> {
        todo!()
    }
}

fn decode_cnode_invocation() {}
// invokeCNodeRevoke函数的Rust版本
fn invoke_cnode_revoke(dest_slot: &mut CTE) -> Exception {
    // 函数实现
    Exception::None
}

// invokeCNodeDelete函数的Rust版本
fn invoke_cnode_delete(dest_slot: &mut CTE) -> Exception {
    // 函数实现
    Exception::None
}

// invokeCNodeCancelBadgedSends函数的Rust版本
fn invoke_cnode_cancel_badged_sends(cap: Cap) -> Exception {
    // 函数实现
    Exception::None
}

fn invoke_cnode_insert(cap: Cap, src_slot: &mut CTE, dest_slot: &mut CTE) -> Exception {
    cte_insert(cap, src_slot, dest_slot);
    Exception::None
}

fn invoke_cnode_move(cap: Cap, src_slot: &mut CTE, dest_slot: &mut CTE) -> Exception {
    cte_move(cap, src_slot, dest_slot);
    Exception::None
}

// invokeCNodeRotate函数的Rust版本
fn invoke_cnode_rotate(
    cap1: Cap,
    cap2: Cap,
    slot1: &mut CTE,
    slot2: &mut CTE,
    slot3: &mut CTE,
) -> Exception {
    // 函数实现
    Exception::None
}

fn set_untyped_cap_as_full(src_cap: Cap, new_cap: Cap, src_slot: &mut CTE) {
    if src_cap.type_equals(CapTag::Untyped.into())
        && new_cap.type_equals(CapTag::Untyped.into())
        && src_cap.cap_ptr() == new_cap.cap_ptr()
        && src_cap.block_size() == new_cap.block_size()
    {
        src_slot
            .cap
            .set_free_index(max_free_index(src_cap.block_size()))
    }
}

/// double linked list insert
/// use unsafe
fn cte_insert(new_cap: Cap, src_slot: &mut CTE, dest_slot: &mut CTE) {
    // copy
    let src_mdb = src_slot.mdb_node;
    let src_cap = src_slot.cap;
    let new_cap_is_revocable = is_cap_revocable(new_cap, src_cap);
    // copy src_mdb's next
    let mut new_mdb = src_mdb;
    new_mdb.set_prev(ptr::from_mut(src_slot) as u64);
    new_mdb.set_revocable(new_cap_is_revocable as u64);
    new_mdb.set_first_badged(new_cap_is_revocable as u64);

    assert!(dest_slot.cap.type_equals(CapTag::Null.into()));
    assert!(dest_slot.mdb_node.prev() == 0 && dest_slot.mdb_node.next() == 0);

    // Prevent parent untyped cap from being used again if creating a child
    // untyped from it.
    set_untyped_cap_as_full(src_cap, new_cap, src_slot);
    dest_slot.cap = new_cap;
    dest_slot.mdb_node = new_mdb;

    src_slot.mdb_node.set_next(ptr::from_mut(dest_slot) as u64);

    // if srcMDB's next is not null, then set its prev to destSlot
    let cte = unsafe { (new_mdb.next() as *mut CTE).as_mut() };
    cte.map(|c| c.mdb_node.set_prev(ptr::from_mut(dest_slot) as u64));
}

/// [TODO] refacator with ownership
fn cte_move(new_cap: Cap, src_slot: &mut CTE, dest_slot: &mut CTE) {
    // cte_move to non-empy destination
    assert!(dest_slot.cap.type_equals(CapTag::Null.into()));
    assert!(dest_slot.is_empty());
    let mdb = src_slot.mdb_node;
    dest_slot.cap = new_cap;
    src_slot.cap = Cap::new_null_cap();
    dest_slot.mdb_node = mdb;
    src_slot.mdb_node = MdbNode::default();

    unsafe {
        // if srcMDB's prev is not null, then set its next to destSlot
        let prev_ptr = mdb.prev() as usize as *mut CTE;
        if !prev_ptr.is_null() {
            (*prev_ptr)
                .mdb_node
                .set_next(ptr::from_mut(dest_slot) as u64);
        }

        // if srcMDB's next is not null, then set its prev to destSlot
        let next_ptr = mdb.next() as usize as *mut CTE;
        if !next_ptr.is_null() {
            (*next_ptr)
                .mdb_node
                .set_prev(ptr::from_mut(dest_slot) as u64);
        }
    }

    // if srcMDB's next is not null, then set its prev to destSlot
}

fn cap_swap_for_delete(slot1: &mut CTE, slot2: &mut CTE) {
    // because of ownership , slot1 != slot2
    cte_swap(slot1.cap, slot1, slot2.cap, slot2)
}

fn cte_swap(cap1: Cap, slot1: &mut CTE, cap2: Cap, slot2: &mut CTE) {
    unsafe {
        slot1.cap = cap2;
        slot2.cap = cap1;

        let mdb1 = slot1.mdb_node;
        let prev_ptr = mdb1.prev() as usize as *mut CTE;
        let next_ptr = mdb1.next() as usize as *mut CTE;

        if !prev_ptr.is_null() {
            (*prev_ptr).mdb_node.set_next(ptr::from_mut(slot2) as u64);
        }

        if !next_ptr.is_null() {
            (*next_ptr).mdb_node.set_prev(ptr::from_mut(slot2) as u64);
        }

        let mdb2 = slot2.mdb_node;
        slot1.mdb_node = mdb2;
        slot2.mdb_node = mdb1;

        let prev_ptr = mdb2.prev() as usize as *mut CTE;
        let next_ptr = mdb2.next() as usize as *mut CTE;

        if !prev_ptr.is_null() {
            (*prev_ptr).mdb_node.set_next(ptr::from_mut(slot1) as u64);
        }

        if !next_ptr.is_null() {
            (*next_ptr).mdb_node.set_prev(ptr::from_mut(slot1) as u64);
        }
    }
}

// cteRevoke函数的Rust版本
fn cte_revoke(slot: &mut CTE) -> Exception {
    // 函数实现
    Exception::None
}

// cteDelete函数的Rust版本
fn cte_delete(slot: &mut CTE, exposed: bool) -> Exception {
    // 函数实现
    Exception::None
}

// cteDeleteOne函数的Rust版本
fn cte_delete_one(slot: &mut CTE) {
    // 函数实现
}

// insertNewCap函数的Rust版本
fn insert_new_cap(parent: &mut CTE, slot: &mut CTE, cap: Cap) {
    // 函数实现
}

// isMDBParentOf函数的Rust版本
fn is_mdbparent_of(cte_a: &CTE, cte_b: &CTE) -> bool {
    // 函数实现
    false
}

// ensureNoChildren函数的Rust版本
fn ensure_no_children(slot: &mut CTE) -> Exception {
    // 函数实现
    Exception::None
}

// ensureEmptySlot函数的Rust版本
fn ensure_empty_slot(slot: &mut CTE) -> Exception {
    // 函数实现
    Exception::None
}

// isFinalCapability函数的Rust版本
fn is_final_capability(cte: &CTE) -> bool {
    // 函数实现
    false
}

// slotCapLongRunningDelete函数的Rust版本
fn slot_cap_long_running_delete(slot: &CTE) -> bool {
    // 函数实现
    false
}

// getReceiveSlots函数的Rust版本
// fn getReceiveSlots(thread: &TCB, buffer: &mut u64) -> *mut CTE {
//     // 函数实现
//     ptr::null_mut::<CTE>()
// }
