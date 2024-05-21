// SPDX-License-Identifier: MPL-2.0

//! This module defines struct `ThreadVM`
//! to represent the layout of user space process virtual memory.
//!
//! The `ProcessVm` struct contains `Vmar`,
//! which stores all existing memory mappings.
//! The `Vm` also contains
//! the basic info of process level vm segments,
//! like init stack and heap.

use alloc::{ffi::CString, vec::Vec};
use anti_rights::Full;

use crate::vm::vmar::Vmar;
use crate::Result;

mod heap;
mod init_stack;

pub use self::{
    heap::Heap,
    heap::USER_HEAP_BASE,
    heap::USER_HEAP_SIZE_LIMIT,
    init_stack::{
        aux_vec::{AuxKey, AuxVec},
        InitStack, InitStackReader, InitStackWriter, INIT_STACK_SIZE, MAX_ARGV_NUMBER, MAX_ARG_LEN,
        MAX_ENVP_NUMBER, MAX_ENV_LEN,
    },
};

/*
 * The user's virtual memory space layout looks like below.
 * TODO: The layout of the userheap does not match the current implementation,
 * And currently the initial program break is a fixed value.
 *
 *  (high address)
 *  +---------------------+ <------+ The top of Vmar, which is the highest address usable
 *  |                     |          Randomly padded pages
 *  +---------------------+ <------+ The base of the initial user stack
 *  | User stack          |
 *  |                     |
 *  +---------||----------+ <------+ The user stack limit, can be extended lower
 *  |         \/          |
 *  | ...                 |
 *  |                     |
 *  | MMAP Spaces         |
 *  |                     |
 *  | ...                 |
 *  |         /\          |
 *  +---------||----------+ <------+ The current program break
 *  | User heap           |
 *  |                     |
 *  +---------------------+ <------+ The original program break
 *  |                     |          Randomly padded pages
 *  +---------------------+ <------+ The end of the program's last segment
 *  |                     |
 *  | Loaded segments     |
 *  | .text, .data, .bss  |
 *  | , etc.              |
 *  |                     |
 *  +---------------------+ <------+ The bottom of Vmar at 0x1_0000
 *  |                     |          64 KiB unusable space
 *  +---------------------+
 *  (low address)
 */

// The process user space virtual memory
pub struct ProcessVm {
    root_vmar: Vmar<Full>,
    init_stack: Option<InitStack>,
    heap: Option<Heap>,
    is_root_server: bool,
}

impl Clone for ProcessVm {
    fn clone(&self) -> Self {
        Self {
            root_vmar: self.root_vmar.dup().unwrap(),
            init_stack: self.init_stack.clone(),
            heap: self.heap.clone(),
            is_root_server: self.is_root_server,
        }
    }
}

impl ProcessVm {
    /// Allocates a new `ProcessVm`
    pub fn alloc(is_root_server: bool) -> Self {
        let root_vmar = Vmar::<Full>::new_root();
        let (init_stack, heap) = if !is_root_server {
            let init_stack = InitStack::new();
            init_stack.alloc_and_map_vmo(&root_vmar).unwrap();
            let heap = Heap::new();
            heap.alloc_and_map_vmo(&root_vmar).unwrap();
            (Some(init_stack), Some(heap))
        } else {
            (None, None)
        };
        Self {
            root_vmar,
            init_stack,
            heap,
            is_root_server,
        }
    }

    /// Forks a `ProcessVm` from `other`.
    ///
    /// The returned `ProcessVm` will have a forked `Vmar`.
    pub fn fork_from(other: &ProcessVm) -> Result<Self> {
        let root_vmar = Vmar::<Full>::fork_from(&other.root_vmar)?;
        Ok(Self {
            root_vmar,
            init_stack: other.init_stack.clone(),
            heap: other.heap.clone(),
            is_root_server: other.is_root_server,
        })
    }

    pub fn root_vmar(&self) -> &Vmar<Full> {
        &self.root_vmar
    }

    /// Returns a reader for reading contents from
    /// the `InitStack`.
    pub fn init_stack_reader(&self) -> Option<InitStackReader> {
        self.init_stack
            .as_ref()
            .map(|stack| stack.reader(&self.root_vmar))
    }

    pub fn init_stack_writer(
        &self,
        argv: Vec<CString>,
        envp: Vec<CString>,
        aux_vec: AuxVec,
    ) -> Option<InitStackWriter> {
        self.init_stack
            .as_ref()
            .map(|stack| stack.writer(&self.root_vmar, argv, envp, aux_vec))
    }

    pub fn heap(&self) -> Option<&Heap> {
        self.heap.as_ref()
    }

    pub fn is_root_server(&self) -> bool {
        self.is_root_server
    }

    /// Clears existing mappings and then maps stack and heap vmo.
    pub fn clear_and_map(&self) {
        self.root_vmar.clear().unwrap();
        self.init_stack
            .as_ref()
            .map(|init_stack| init_stack.alloc_and_map_vmo(&self.root_vmar).unwrap());
        self.heap
            .as_ref()
            .map(|heap| heap.alloc_and_map_vmo(&self.root_vmar).unwrap());
    }
}
