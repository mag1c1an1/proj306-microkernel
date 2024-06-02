//! Root Server is the first user thread in sel4.
//! It has the ownership of all kernel resources.

use aster_frame::boot::initramfs;
use aster_frame::sync::SpinLock;
use spin::Once;

use crate::common::region::{KAddr, Region};
use crate::root_server::elf::load_elf;

mod elf;

static ROOT_SERVER: Once<SpinLock<RawRootServer>> = Once::new();

#[repr(C)]
pub struct RawRootServer {
    pub cnode: KAddr,
    pub vspace: KAddr,
    pub asid_pool: KAddr,
    pub ipc_buf: KAddr,
    pub boot_info: KAddr,
    pub extra_bi: KAddr,
    pub tcb: KAddr,
    pub paging: Region,
}

pub fn create_root_thread() {
    let image = initramfs();
    error!("{}",image.len());
    load_elf(image).unwrap();
}

mod test {
    use ktest::ktest;

    use crate::root_server::RawRootServer;

    #[ktest]
    fn size_test() {
        assert_eq!(core::mem::size_of::<RawRootServer>(), 72);
    }
}