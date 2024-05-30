#![no_std]

#[macro_use]
extern crate log;

pub mod cspace;
pub mod sel4;
// pub mod syscall;
pub mod thread;
pub mod utils;
pub mod vspace;

// pub mod error;

// pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub fn init() {}
