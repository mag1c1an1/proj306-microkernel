#![no_std]
#![allow(dead_code)]
#![feature(core_intrinsics)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(let_chains)]
#![feature(btree_cursors)]
#![feature(unchecked_math)]
// FIXME: This feature is used to support vm capbility now as a work around.
// Since this is an incomplete feature, use this feature is unsafe.
// We should find a proper method to replace this feature with min_specialization, which is a sound feature.
#![feature(specialization)]
#![feature(const_option)]
#![feature(generic_const_exprs)]
#![allow(warnings)]

use align_ext::AlignExt;
use alloc::{
    sync::Arc,
    vec::{self, Vec},
};
use anti_frame::{
    arch::qemu::{exit_qemu, QemuExitCode},
    boot::initramfs,
    cpu::UserContext,
    sync::Mutex,
    task::{Task, TaskOptions},
    user::{UserEvent, UserMode, UserSpace},
    vm::{PageFlags, Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmSpace, PAGE_SIZE},
};
use error::Error;
use process::Process;
use thread::{task::create_new_user_task, Thread};

use crate::{
    thread::kernel_thread::{KernelThreadExt, ThreadOptions},
    vm::vmo::Vmo,
};

#[macro_use]
extern crate log;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

// mod boot;
pub(crate) mod sel4;
// mod common;
// mod config;
// mod cspace;
// // mod debug;
// // mod deps;
pub mod error;

// mod exception;
// mod interrupt;
// mod kernel;
// pub(crate) mod loader;
// mod object;
// #[cfg(feature = "ENABLE_SMP")]
// mod smp;
// // mod structures;
mod syscall;
// // mod task_manager;
// #[cfg(feature = "ENABLE_UINTC")]
// mod uintc;
// #[cfg(feature = "ENABLE_UINTC")]
// mod uintr;
// mod utils;
// mod vspace;
pub mod process;
pub mod sched;
pub mod thread;
pub mod utils;
pub mod vm;

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

/// handle can be used map to a vspace
pub struct Handle {}

pub struct RootServer {}

pub fn init() {
    sched::init();
}

pub fn root_server() {
    let elf_binary = initramfs();
    let argv = Vec::new();
    let envp = Vec::new();
    let _ = Process::spawn_user_process(elf_binary, argv, envp, true);
    exit_qemu(QemuExitCode::Success);
}

pub fn run_root_server() -> ! {
    Thread::spawn_kernel_thread(ThreadOptions::new(root_server));
    unreachable!()
}

pub fn debug() {
    self::sel4::boot::bootstrap::init_sys_state();
}
