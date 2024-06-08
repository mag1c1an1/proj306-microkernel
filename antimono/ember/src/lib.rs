#![no_std]
#![feature(error_in_core)]
#![feature(generic_const_exprs)]
#![feature(non_null_convenience)]
#![feature(strict_provenance)]
extern crate alloc;
#[macro_use]
extern crate log;

pub use error::EmberResult;
use crate::root_server::SysLauncher;

pub mod cspace;
pub mod sel4;
pub mod syscall;
pub mod thread;
pub mod utils;
pub mod vspace;
pub mod common;
pub mod error;
pub mod mm;
pub mod object;
pub mod root_server;
mod sched;

pub mod boot_info;

pub fn init() {
    sched::init();
}


pub fn run_root_server() {
    SysLauncher::new().launch()
}