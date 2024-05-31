#![no_std]
#![feature(error_in_core)]
extern crate alloc;
#[macro_use]
extern crate log;

pub use error::Result;

pub mod cspace;
pub mod sel4;
pub mod syscall;
pub mod thread;
pub mod utils;
pub mod vspace;

pub mod common;
pub mod error;

pub fn init() {}
