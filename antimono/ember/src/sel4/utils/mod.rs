use anti_frame::vm::kspace::paddr_to_vaddr;

use crate::{sel4::CONFIG_ROOT_CNODE_SIZE_BITS, BIT, ROUND_DOWN, ROUND_UP};

pub mod macros;

use super::{
    common::{p_region_t, region_t, v_region_t}, cspace::{cap_t, cte::cte_t, mdb::mdb_node_t}, seL4_PageBits, vspace::{pptr_t, pte_t, vptr_t}, CONFIG_PT_LEVELS, PADDR_TOP, PPTR_BASE, PPTR_TOP
};

pub fn MAX_FREE_INDEX(bits: usize) -> usize {
    // BIT!(bits - seL4_MinUntypedBits)
    todo!()
}

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

#[no_mangle]
#[inline]
pub fn pageBitsForSize(page_size: usize) -> usize {
    match page_size {
        _ => panic!("Invalid page size!"),
    }
}

#[inline]
pub fn cpu_id() -> usize {
    #[cfg(feature = "ENABLE_SMP")]
    {
        use crate::smp::get_currenct_cpu_index;
        // unsafe { getCurrentCPUIndex() }
        get_currenct_cpu_index()
    }
    #[cfg(not(feature = "ENABLE_SMP"))]
    {
        0
    }
}
#[inline]
pub fn is_reg_empty(reg: &region_t) -> bool {
    reg.start == reg.end
}

pub fn paddr_to_pptr(p: usize) -> usize {
    todo!()
}
pub fn pptr_to_paddr(p: usize) -> usize {
    todo!()
}

#[inline]
pub fn paddr_to_pptr_reg(reg: &p_region_t) -> region_t {
    region_t {
        start: paddr_to_pptr(reg.start),
        end: paddr_to_pptr(reg.end),
    }
}

pub fn ceiling_kernel_window(mut p: usize) -> usize {
    if pptr_to_paddr(p) > PADDR_TOP {
        p = PPTR_TOP;
    }
    p
}

#[inline]
pub fn pptr_to_paddr_reg(reg: region_t) -> p_region_t {
    p_region_t {
        start: pptr_to_paddr(reg.start),
        end: pptr_to_paddr(reg.end),
    }
}

pub fn pptr_in_kernel_window(pptr: usize) -> bool {
    pptr >= PPTR_BASE && pptr < PPTR_TOP
}

#[inline]
pub fn get_n_paging(v_reg: v_region_t, bits: usize) -> usize {
    let start = ROUND_DOWN!(v_reg.start, bits);
    let end = ROUND_UP!(v_reg.end, bits);
    (end - start) / BIT!(bits)
}

pub fn arch_get_n_paging(it_v_reg: v_region_t) -> usize {
    todo!()
    // let mut n: usize = 0;
    // for i in 0..CONFIG_PT_LEVELS - 1 {
    //     n += get_n_paging(it_v_reg, RISCV_GET_LVL_PGSIZE_BITS(i));
    // }
    // return n;
}

/// ptr is paddr
pub fn write_slot(ptr: usize, cap: cap_t) {
    unsafe {
        // contert it to kernel vaddr
        let ptr = paddr_to_vaddr(ptr) as *mut cte_t;
        (*ptr).cap = cap;
        (*ptr).cteMDBNode = mdb_node_t::default();
        let mdb = &mut (*ptr).cteMDBNode;
        mdb.set_revocable(1);
        mdb.set_first_badged(1);
    }
}
