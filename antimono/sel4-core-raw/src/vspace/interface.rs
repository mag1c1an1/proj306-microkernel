use crate::{common::{sel4_config::*, structures::exception_t, utils::{convert_to_mut_type_ref, pageBitsForSize}, fault::*}, BIT, ROUND_DOWN};
use crate::cspace::interface::{cap_t, CapTag};
use core::intrinsics::unlikely;
use super::pte::pte_t;
use super::utils::{RISCV_GET_PT_INDEX, RISCV_GET_LVL_PGSIZE, RISCV_GET_LVL_PGSIZE_BITS, kpptr_to_paddr};

use super::{asid::{find_vspace_for_asid, asid_t}, utils::pptr_to_paddr, structures::{vptr_t, pptr_t}};

#[no_mangle]
#[link_section = ".page_table"]
pub static mut kernel_root_pageTable: [pte_t; BIT!(PT_INDEX_BITS)] =
    [pte_t { words: [0] }; BIT!(PT_INDEX_BITS)];

#[no_mangle]
#[link_section = ".page_table"]
pub static mut kernel_image_level2_pt: [pte_t; BIT!(PT_INDEX_BITS)] =
    [pte_t { words: [0] }; BIT!(PT_INDEX_BITS)];


#[no_mangle]
pub fn rust_map_kernel_window() {
    let mut pptr = PPTR_BASE;

    let mut paddr = PADDR_BASE;
    while pptr < PPTR_TOP {
        unsafe {
            kernel_root_pageTable[RISCV_GET_PT_INDEX(pptr, 0)] = pte_t::pte_next(paddr, true);
        }
        pptr += RISCV_GET_LVL_PGSIZE(0);
        paddr += RISCV_GET_LVL_PGSIZE(0);
    }
    pptr = ROUND_DOWN!(KERNEL_ELF_BASE, RISCV_GET_LVL_PGSIZE_BITS(0));
    paddr = ROUND_DOWN!(KERNEL_ELF_PADDR_BASE, RISCV_GET_LVL_PGSIZE_BITS(0));
    unsafe {
        kernel_root_pageTable[RISCV_GET_PT_INDEX(KERNEL_ELF_PADDR_BASE + PPTR_BASE_OFFSET, 0)] =
        pte_t::pte_next(
                kpptr_to_paddr(kernel_image_level2_pt.as_ptr() as usize),
                false,
            );
        kernel_root_pageTable[RISCV_GET_PT_INDEX(pptr, 0)] = pte_t::pte_next(
            kpptr_to_paddr(kernel_image_level2_pt.as_ptr() as usize),
            false,
        );
    }

    let mut index = 0;
    while pptr < PPTR_TOP + RISCV_GET_LVL_PGSIZE(0) {
        unsafe {
            kernel_image_level2_pt[index] = pte_t::pte_next(paddr, true);
        }
        pptr += RISCV_GET_LVL_PGSIZE(1);
        paddr += RISCV_GET_LVL_PGSIZE(1);
        index += 1;
    }
}

#[inline]
pub fn activate_kernel_vspace() {
    unsafe {
        // setVSpaceRoot(kpptr_to_paddr(kernel_root_pageTable.as_ptr() as usize), 0);
    }
}

#[no_mangle]
pub fn copyGlobalMappings(Lvl1pt: usize) {
    let mut i: usize = RISCV_GET_PT_INDEX(0x80000000, 0);
    while i < BIT!(PT_INDEX_BITS) {
        unsafe {
            let newLvl1pt = (Lvl1pt + i * 8) as *mut usize;
            *newLvl1pt = kernel_root_pageTable[i].words[0];
            i += 1;
        }
    }
}

#[no_mangle]
pub fn set_vm_root(vspace_root: &cap_t) -> Result<(), lookup_fault_t> {
    if vspace_root.get_cap_type() != CapTag::CapPageTableCap {
        unsafe {
            // setVSpaceRoot(kpptr_to_paddr(kernel_root_pageTable.as_ptr() as usize), 0);
            return Ok(());
        }
    }
    let lvl1pt = convert_to_mut_type_ref::<pte_t>(vspace_root.get_pt_base_ptr());
    let asid = vspace_root.get_pt_mapped_asid();
    let find_ret = find_vspace_for_asid(asid);
    let mut ret = Ok(());
    if unlikely(
        find_ret.status != exception_t::EXCEPTION_NONE || find_ret.vspace_root.is_none() || find_ret.vspace_root.unwrap() != lvl1pt,
    ) {
        unsafe {
            if let Some(lookup_fault) = find_ret.lookup_fault {
                ret = Err(lookup_fault);
            }
            // setVSpaceRoot(kpptr_to_paddr(kernel_root_pageTable.as_ptr() as usize), 0);
        }
    }
    // setVSpaceRoot(pptr_to_paddr(lvl1pt as *mut pte_t as usize), asid);
    ret
}


#[no_mangle]
pub fn unmapPage(page_size: usize, asid: asid_t, vptr: vptr_t, pptr: pptr_t) -> Result<(), lookup_fault_t> {
    let find_ret = find_vspace_for_asid(asid);
    if find_ret.status != exception_t::EXCEPTION_NONE {
        return Err(find_ret.lookup_fault.unwrap());
    }

    let lu_ret = unsafe {(*find_ret.vspace_root.unwrap()).lookup_pt_slot(vptr)};

    if lu_ret.ptBitsLeft != pageBitsForSize(page_size) {
        return Ok(());
    }

    let slot = unsafe {&(*lu_ret.ptSlot)};

    if slot.get_vaild() == 0 || slot.is_pte_table() || slot.get_ppn() << seL4_PageBits != pptr_to_paddr(pptr) {
        return Ok(());
    }

    unsafe {
        let slot = lu_ret.ptSlot as *mut usize;
        *slot = 0;
    }
    Ok(())
}