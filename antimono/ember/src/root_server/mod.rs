//! Root Server is the first user thread in sel4.
//! It has the ownership of all kernel resources.

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::ops::Range;
use core::ptr::NonNull;

use aster_frame::boot::initramfs;
use aster_frame::cpu::UserContext;
use aster_frame::sync::SpinLock;
use aster_frame::task::{Task, TaskOptions};
use aster_frame::user::UserSpace;
use aster_frame::vm::{PageFlags, Vaddr, VmFrameVec, VmMapOptions, VmSpace};
use spin::Once;

use sel4::sys::seL4_TCBBits;

use crate::bit;
use crate::common::region::{Kaddr, Region};
use crate::root_server::elf::{ElfLoadInfo, load_elf};
use crate::thread::{task, TcbObject, Thread};

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
    let mut cpu_ctx = UserContext::default();

    let vm_space = Arc::new(VmSpace::new());

    for desc in ui.descs {
        let mut map = VmMapOptions::new();
        map.addr(Some(desc.start))
            .flags(desc.pt_flags);
        vm_space.map(desc.segment, &map).unwrap();
    }

    cpu_ctx.set_rip(ui.elf_load_info.entry_point() as _);
    // cpu_ctx.set_rdi(vptr);
    let user_space = Arc::new(UserSpace::new(vm_space.clone(), cpu_ctx));
    // let thread_name = Some(ThreadName::new_from_executable_path(executable_path)?);
    let thread = Arc::new_cyclic(|thread_ref| unsafe {
        let task = task::create_new_user_task(user_space, thread_ref.clone());
        let mut m = Box::new([0u8; bit!(seL4_TCBBits)]);
        let tcb_object = TcbObject::new(NonNull::new(m.as_mut_ptr()).unwrap());
        Thread::new(task, tcb_object, vm_space.clone())
    });

    thread.run();
}

mod test {
    use ktest::ktest;

    use crate::root_server::RawRootServer;

    #[ktest]
    fn size_test() {
        assert_eq!(core::mem::size_of::<RawRootServer>(), 72);
    }
}