// x86 page table related
#![allow(non_camel_case_types)]

use crate::{BIT, ROUND_DOWN, ROUND_UP};
use crate::sel4::common::v_region_t;
use super::{cspace::cap_t, exception::exception_t};

pub type pptr_t = usize; // kernel virtual address
pub type paddr_t = usize; // phyical address
pub type vptr_t = usize; // user virtual address

pub const IT_ASID: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct findVSpaceForASID_ret_t {
    pub status: exception_t,
    pub vspace_root: *mut pte_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct lookupPTSlot_ret_t {
    pub ptSlot: *mut pte_t,
    pub ptBitsLeft: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct lookupPDSlot_ret_t {
    pub status: exception_t,
    pub pdSlot: *mut pde_t,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct pml4e_t {
    pub words: [u64; 1],
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct pdpte_t {
    pub words: [u64; 1],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct pde_t {
    pub words: [u64; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct pte_t {
    pub words: [usize; 1],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct vm_attributes_t {
    pub words: [usize; 1],
}

impl vm_attributes_t {
    pub fn new(value: usize) -> Self {
        Self {
            words: [value & 0x1usize],
        }
    }
    pub fn from_word(w: usize) -> Self {
        Self { words: [w] }
    }
    pub fn get_execute_never(&self) -> usize {
        self.words[0] & 0x1usize
    }
    pub fn set_execute_never(&mut self, v64: usize) {
        self.words[0] &= !0x1usize;
        self.words[0] |= (v64 << 0) & 0x1usize;
    }
}

pub fn init_boot_pd() {
    todo!()
}

pub fn enable_paging() {
    todo!()
}

pub fn map_kernel_window(
    num_ioapic: u32,
    ioapic_paddrs: paddr_t,
    num_drhu: u32,
    drhu_list: paddr_t,
) -> bool {
    todo!()
}

// pub fn  map_skim_window(vptr_t skim_start, vptr_t skim_end)->bool{todo!()}
pub fn map_kernel_window_devices(
    pt: *mut pte_t,
    num_ioapic: u32,
    ioapic_paddrs: *mut paddr_t,
    num_drhu: u32,
    drhu_list: *mut paddr_t,
) -> bool {
    todo!()
}

// pub fn init_tss(tss: *mut tss_t) {
//     todo!()
// }
// pub fn init_gdt(gdt_entry_t *gdt, tss_t *tss){todo!()}
// pub fn init_idt_entry(idt_entry_t *idt, interrupt_t interrupt, (*handler)()){todo!()}
// pub fn getValidNativeRoot(vspace_cap: cap_t) -> *mut vspace_root_t {
//     todo!()
// }
pub fn get_boot_pd() -> *mut pde_t {
    todo!()
}

pub fn map_temp_boot_page(entry: usize, large_pages: u32) {
    todo!()
}

pub fn init_vm_state() -> bool {
    todo!()
}

pub fn init_dtrs() {
    todo!()
}

pub fn map_it_pt_cap(vspace_cap: cap_t, pt_cap: &cap_t) {
    todo!()
    // let vptr = _pt_cap.get_pt_mapped_address();
    // let lvl1pt = convert_to_mut_type_ref::<pte_t>(_vspace_cap.get_cap_ptr());
    // let pt = _pt_cap.get_cap_ptr();
    // let pt_ret = lvl1pt.lookup_pt_slot(vptr);
    // let targetSlot = convert_to_mut_type_ref::<cte_t>(pt_ret.ptSlot as usize);
    // *targetSlot = pte_t::new(
    //     pptr_to_paddr(pt) >> seL4_PageBits,
    //     0,
    //     0,
    //     0,
    //     0,
    //     0,
    //     0,
    //     0,
    //     0,
    //     1,
    // );
}

pub fn map_it_pd_cap(vspace_cap: cap_t, pd_cap: cap_t) {
    todo!()
}

pub fn map_it_frame_cap(pd_cap: cap_t, frame_cap: cap_t) {
    todo!()
}

pub fn map_it_pdpt_cap(vspace_cap: cap_t, pdpt_cap: cap_t) {}
//  pub fn write_it_asid_pool(cap_t it_ap_cap, cap_t it_vspace_cap){todo!()}
// pub fn init_pat_msr()->bool{todo!()}
// pub fn create_it_address_space(cap_t root_cnode_cap, v_region_t it_v_reg)->cap_t{todo!()}

pub fn is_vtable_root(cap: cap_t) -> bool {
    todo!()
}

pub fn lookupPTSlot() -> lookupPTSlot_ret_t {
    todo!()
}


pub fn arch_get_n_paging() {}

pub fn get_n_paging(v_reg: v_region_t, bits: usize) -> usize {
    let start = ROUND_DOWN!(v_reg.start,bits);
    let end = ROUND_UP!(v_reg.end,bits);
    (end - start) / BIT!(bits)
}
