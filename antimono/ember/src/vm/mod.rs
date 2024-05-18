use crate::{
    sel4::{
        vspace::{pde_t, pdpte_t, pml4e_t, pte_t},
        PDPT_INDEX_BITS, PD_INDEX_BITS, PML4_INDEX_BITS, PT_INDEX_BITS,
    },
    BIT,
};

pub mod page_fault_handler;
pub mod perms;
pub mod vmar;
pub mod vmo;

// the privileged kernel mapping PD&PT
#[no_mangle]
pub static mut x64KSKernelPML4: [pml4e_t; BIT!(PML4_INDEX_BITS)] =
    [pml4e_t { words: [0; 1] }; BIT!(PML4_INDEX_BITS)];

#[no_mangle]
pub static mut x64KSKernelPDPT: [pdpte_t; BIT!(PDPT_INDEX_BITS)] =
    [pdpte_t { words: [0; 1] }; BIT!(PDPT_INDEX_BITS)];

#[no_mangle]
pub static mut x64KSKernelPD: [pde_t; BIT!(PD_INDEX_BITS)] =
    [pde_t { words: [0; 1] }; BIT!(PD_INDEX_BITS)];

#[no_mangle]
pub static mut x64KSKernelPT: [pte_t; BIT!(PT_INDEX_BITS)] =
    [pte_t { words: [0; 1] }; BIT!(PT_INDEX_BITS)];


