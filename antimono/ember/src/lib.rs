#![no_std]
#![feature(error_in_core)]
#![feature(generic_const_exprs)]
#![feature(non_null_convenience)]
#![feature(strict_provenance)]
extern crate alloc;
#[macro_use]
extern crate log;

pub use error::EmberResult;

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

pub fn init() {
    sched::init();
}


pub fn run_root_server() {
    root_server::create_root_thread();
}