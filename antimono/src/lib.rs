#![no_std]
#![forbid(unsafe_code)]

use aster_frame::prelude::*;

#[aster_main]
fn kernel_main() {
    set_max_level(option_env!("LOG").unwrap_or(""));
    ember::init();
    println!("Hello world from guest kernel!");
    log::trace!("xxxx");
    log::warn!("xxxx");
    log::error!("xxxx");
    log::info!("xxxx");
    log::debug!("xxxx");
}
