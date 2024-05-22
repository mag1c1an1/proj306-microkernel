// x86 page table related
#![allow(non_camel_case_types)]

pub type pptr_t = usize; // kernel virtual address
pub type paddr_t = usize; // phyical address
pub type vptr_t = usize; // user virtual address

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
