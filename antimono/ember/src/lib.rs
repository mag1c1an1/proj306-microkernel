#![no_std]
// #![crate_type = "staticlib"]
// #![no_main]
#![allow(dead_code)]
#![feature(core_intrinsics)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(linkage)]
// #![feature(generic_const_exprs)]

// use align_ext::AlignExt;
// use anti_frame::{boot::initramfs, early_println, vm::MEMORY_REGIONS};
// use config::USER_TOP;
// use structures::{p_region_t, v_region_t};
// use crate::{
//     boot::try_init_kernel,
//     common::sel4_config::PAGE_BITS,
//     config::{BI_FRAME_SIZE_BITS, PAGE_SIZE},
//     loader::Elf,
// };

use align_ext::AlignExt;
use alloc::{sync::Arc, vec};
use anti_frame::{
    cpu::UserContext,
    task::{Task, TaskOptions},
    user::{UserEvent, UserMode, UserSpace},
    vm::{PageFlags, Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmSpace, PAGE_SIZE},
};
use sel4::create_frames_of_region_ret_t;
use thread::{task::create_new_user_task, user::create_root_task_space, Thread};

use crate::thread::kernel_thread::{KernelThreadExt, ThreadOptions};

#[macro_use]
extern crate log;

extern crate alloc;

mod boot;
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
pub(crate) mod loader;
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
pub mod sched;
pub mod thread;
pub mod vm;

// struct ui_info_t {
//     /// region where the user image lies in
//     pub p_reg: p_region_t,
//     /// UI virtual address + pv_offset = UI physical address
//     pub pv_offset: isize,
//     /// entry point
//     pub v_entry: usize,
// }

// fn arch_init_freemem(ui_p_reg: p_region_t, it_v_reg: v_region_t) -> bool {
//     todo!();
// }

// fn create_rootserver_objects() {
//     todo!()
// }
pub fn init() {
    sched::init();
}

pub fn run_root_task() {
    let r_space = create_root_task_space();
    // let t = current_thread!();
    let task = create_new_user_task(Arc::new(r_space));
    task.run();
    // unreachable!()
}
