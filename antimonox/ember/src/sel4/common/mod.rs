use crate::sel4::vspace::{paddr_t, pptr_t, vptr_t};
use anti_frame::vm::kspace::LINEAR_MAPPING_BASE_VADDR;

use super::cspace::{cap_t, CapTag};

/// kernel virtual memory region
#[derive(Copy, Clone)]
pub struct region_t {
    pub start: pptr_t,
    pub end: pptr_t,
}

/// physical virtual memory region
#[derive(Copy, Clone, Debug)]
pub struct p_region_t {
    pub start: paddr_t,
    pub end: paddr_t,
}

/// user virtual memory region
#[derive(Copy, Clone)]
pub struct v_region_t {
    pub start: vptr_t,
    pub end: vptr_t,
}

pub fn paddr_to_pptr(addr: paddr_t) -> pptr_t {
    addr + LINEAR_MAPPING_BASE_VADDR
}
pub fn paddr_to_pptr_reg(reg: &p_region_t) -> region_t {
    region_t {
        start: paddr_to_pptr(reg.start),
        end: paddr_to_pptr(reg.end),
    }
}

pub fn pptr_to_paddr(pptr: pptr_t) -> paddr_t {
    pptr - LINEAR_MAPPING_BASE_VADDR
}

pub fn pptr_to_paddr_reg(reg: region_t) -> p_region_t {
    p_region_t {
        start: pptr_to_paddr(reg.start),
        end: pptr_to_paddr(reg.end),
    }
}

pub fn pptr_of_cap(cap: &cap_t) -> pptr_t {
    if cap.get_cap_type() == CapTag::CapNullCap {
        panic!("wrong cap get ptr");
    }
    cap.get_cap_ptr()
}
