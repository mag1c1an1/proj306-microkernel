pub mod interface;
mod asid;
mod pte;
mod satp;
mod structures;
mod utils;
mod vm_rights;

pub use structures::*;
pub use interface::{activate_kernel_vspace, rust_map_kernel_window, copyGlobalMappings, set_vm_root, unmapPage};
pub use vm_rights::{VMReadWrite, VMReadOnly, maskVMRights};
pub use asid::{
    asid_t, asid_pool_t, riscvKSASIDTable, delete_asid_pool, delete_asid,
    find_vspace_for_asid, get_asid_pool_by_index, set_asid_pool_by_index
};
pub use utils::{pptr_to_paddr, paddr_to_pptr, kpptr_to_paddr, RISCV_GET_LVL_PGSIZE_BITS, RISCV_GET_LVL_PGSIZE, checkVPAlignment};
pub use pte::pte_t;

#[inline]
#[no_mangle]
pub fn setVSpaceRoot(addr: paddr_t, asid: usize) {
    // todo
    todo!()
}