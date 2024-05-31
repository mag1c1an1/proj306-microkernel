#[macro_use]
mod macros;

// use crate::{
//     sel4::{seL4_MinUntypedBits, CONFIG_ROOT_CNODE_SIZE_BITS},
//     BIT, ROUND_DOWN, ROUND_UP,
// };
//
//
// use super::{
//     common::{p_region_t, region_t, v_region_t},
//     cspace::{cap_t, cte::cte_t, mdb::mdb_node_t},
//     seL4_PageBits,
//     vspace::{pptr_t, pte_t, vptr_t},
//     CONFIG_PT_LEVELS, PADDR_TOP, PPTR_BASE, PPTR_TOP,
// };

// pub fn MAX_FREE_INDEX(bits: usize) -> usize {
//     BIT!(bits - seL4_MinUntypedBits)
// }

#[inline]
pub fn convert_to_type_ref<T>(addr: usize) -> &'static T {
    assert_ne!(addr, 0);
    unsafe { &*(addr as *mut T) }
}

#[inline]
pub fn convert_to_mut_type_ref<T>(addr: usize) -> &'static mut T {
    assert_ne!(addr, 0);
    unsafe { &mut *(addr as *mut T) }
}

#[inline]
pub fn convert_to_mut_type_ref_unsafe<T>(addr: usize) -> &'static mut T {
    unsafe { &mut *(addr as *mut T) }
}

#[inline]
pub fn convert_to_option_type_ref<T>(addr: usize) -> Option<&'static T> {
    if addr == 0 {
        return None;
    }
    Some(convert_to_type_ref::<T>(addr))
}

#[inline]
pub fn convert_to_option_mut_type_ref<T>(addr: usize) -> Option<&'static mut T> {
    if addr == 0 {
        return None;
    }
    Some(convert_to_mut_type_ref::<T>(addr))
}

#[inline]
pub fn page_bits_for_size(page_size: usize) -> usize {
    match page_size {
        _ => panic!("Invalid page size!"),
    }
}

#[inline]
pub fn cpu_id() -> usize {
    0
}

// pub fn is_reg_empty(reg: &region_t) -> bool {
//     reg.start == reg.end
// }
// pub fn is_v_reg_empty(reg: &v_region_t) -> bool {
//     reg.start == reg.end
// }
//
// /// ptr is paddr
// pub fn write_slot(ptr: *mut cte_t, cap: cap_t) {
//     unsafe {
//         (*ptr).cap = cap;
//         (*ptr).cteMDBNode = mdb_node_t::default();
//         let mdb = &mut (*ptr).cteMDBNode;
//         mdb.set_revocable(1);
//         mdb.set_first_badged(1);
//     }
// }
