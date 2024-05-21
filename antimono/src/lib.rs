#![no_std]
// #![forbid(unsafe_code)]

extern crate alloc;
use anti_frame::prelude::*;

#[aster_main]
fn kernel_main() {
    ember::init();
    ember::run_root_server();
}
