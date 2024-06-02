//! Root Server is the first user thread in sel4.
//! It has the ownership of all kernel resources.

use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::ops::Range;

use aster_frame::boot::initramfs;
use aster_frame::sync::SpinLock;
use aster_frame::vm::{PageFlags, Vaddr, VmFrameVec};
use spin::Once;

use crate::common::region::{Kaddr, Region};
use crate::root_server::elf::{ElfLoadInfo, load_elf};

mod elf;

static ROOT_SERVER: Once<SpinLock<RawRootServer>> = Once::new();

#[repr(C)]
pub struct RawRootServer {
    pub cnode: Kaddr,
    pub vspace: Kaddr,
    pub asid_pool: Kaddr,
    pub ipc_buf: Kaddr,
    pub boot_info: Kaddr,
    pub extra_bi: Kaddr,
    pub tcb: Kaddr,
    pub paging: Region,
}

#[derive(Debug)]
pub struct UserImage {
    // elf segments and its virtual address
    pub descs: Vec<SegmentDesc>,
    pub bounds: Range<Vaddr>,
    pub elf_load_info: ElfLoadInfo,
}

impl Display for UserImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("\nentry: 0x{:x}\n", self.elf_load_info.entry_point()))?;
        f.write_fmt(format_args!("bounds: [0x{:x}, 0x{:x})\n", self.bounds.start, self.bounds.end))?;
        for desc in self.descs.iter() {
            f.write_fmt(format_args!("{}\n", desc))?
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct SegmentDesc {
    pub segment: VmFrameVec,
    pub start: Vaddr,
    pub pt_flags: PageFlags,
}

impl Display for SegmentDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("[0x{:x}, 0x{:x}) {:?}", self.start, self.start + self.segment.nbytes(), self.pt_flags))
    }
}


pub fn create_root_thread() {
    let image = initramfs();
    error!("{}",image.len());
    let ui = load_elf(image).unwrap();
    trace!("{}",ui);
}

mod test {
    use ktest::ktest;

    use crate::root_server::RawRootServer;

    #[ktest]
    fn size_test() {
        assert_eq!(core::mem::size_of::<RawRootServer>(), 72);
    }
}