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

use align_ext::AlignExt;
use anti_frame::{boot::initramfs, early_println, vm::MEMORY_REGIONS};
use config::USER_TOP;
use structures::{p_region_t, v_region_t};
use xmas_elf::program::ProgramHeader;

use crate::{
    boot::try_init_kernel, common::sel4_config::PAGE_BITS, config::{BI_FRAME_SIZE_BITS, PAGE_SIZE}, loader::Elf
};

#[macro_use]
extern crate log;

extern crate alloc;

#[cfg(feature = "ENABLE_UINTC")]
mod async_runtime;
mod boot;
mod c_api;
mod common;
mod config;
mod cspace;
mod debug;
mod deps;
pub mod error;
mod exception;
mod interrupt;
mod kernel;
pub(crate) mod loader;
mod object;
#[cfg(feature = "ENABLE_SMP")]
mod smp;
mod structures;
mod syscall;
mod task_manager;
#[cfg(feature = "ENABLE_UINTC")]
mod uintc;
#[cfg(feature = "ENABLE_UINTC")]
mod uintr;
mod utils;
mod vspace;

#[no_mangle]
pub extern "C" fn halt() {
    todo!()
    // shutdown()
}

#[no_mangle]
pub extern "C" fn strnlen(str: *const u8, _max_len: usize) -> usize {
    unsafe {
        let mut c = str;
        let mut ans = 0;
        while (*c) != 0 {
            ans += 1;
            c = c.add(1);
        }
        ans
    }
}

struct ui_info_t {
    /// region where the user image lies in
    pub p_reg: p_region_t,
    /// UI virtual address + pv_offset = UI physical address
    pub pv_offset: isize,
    /// entry point
    pub v_entry: usize,
}

fn arch_init_freemem(ui_p_reg: p_region_t, it_v_reg: v_region_t) -> bool {
    todo!();
}

fn create_rootserver_objects() {
    todo!()
}

pub fn init() {
    trace!("in sel4");
    // get user land image
    let user_image = initramfs();
    trace!("user_image len : {}", user_image.len());
    let x = MEMORY_REGIONS.get().unwrap();
    trace!("{:#?}", x);
    let elf = Elf::parse_elf(user_image).unwrap();
    let v_entry = elf.entry_point();
    let ui_v_reg = elf.memory_bounds();
    trace!("ELF_loading userland images from boot modules:");
    trace!(
        "size=0x{:x} v_entry=0x{:x} v_start=0x{:x}, v_end=0x{:x}",
        ui_v_reg.end - ui_v_reg.start,
        v_entry,
        ui_v_reg.start,
        ui_v_reg.end,
    );
    assert!(
        ui_v_reg.start % PAGE_SIZE == 0,
        "Userland image virtual start address must be page aligned"
    );
    // for ipc buffer frame and bootinfo frame, need 2 * 4K of addditional userland virtual memory
    assert!(
        ui_v_reg.end + 2 * (1 << PAGE_BITS) <= 0x7FFFFFFFFFFF,
        "Userland image virtual end address is too high"
    );
    assert!(
        v_entry >= ui_v_reg.start && v_entry < ui_v_reg.end,
        "Userland imgae entry point does not lie within userland image"
    );
    try_init_kernel(ui_v_reg, 0, v_entry);
    trace!("drop to user space");
}
