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


// mod config;
// mod boot;
// mod object;
// mod interrupt;
// mod exception;
// mod common;
// mod task_manager;
// mod debug;
// mod vspace;
// mod cspace;
// mod deps;
// mod utils;
// mod syscall;
mod structures;
// mod kernel;
// mod c_api;
#[cfg(feature = "ENABLE_SMP")]
mod smp;
#[cfg(feature = "ENABLE_UINTC")]
mod uintc;
#[cfg(feature = "ENABLE_UINTC")]
mod uintr;
#[cfg(feature = "ENABLE_UINTC")]
mod async_runtime;


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

pub fn init() {}