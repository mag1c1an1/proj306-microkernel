use core::ptr;

use crate::{
    error::Exception,
    structures::{Cap, CapTag, MdbNode},
};

use super::{is_cap_revocable, untyped::max_free_index};

#[derive(Debug, Clone, Copy)]
/// cap table entry
pub struct CTE {
    pub cap: Cap,
    pub mdb_node: MdbNode,
}

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
// fn getReceiveSlots(thread: &TCB, buffer: &mut u64) -> *mut CTE {
//     // 函数实现
//     ptr::null_mut::<CTE>()
// }
