#![no_std]
#![forbid(unsafe_code)]

use aster_frame::prelude::*;

#[aster_main]
fn kernel_main() {
    set_max_level(option_env!("LOG").unwrap_or("TRACE"));
    ember::init();
    ember::run_root_server();
}
