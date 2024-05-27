use align_ext::AlignExt;
use alloc::sync::Arc;
use anti_frame::{
    boot::initramfs,
    cpu::UserContext,
    user::UserSpace,
    vm::{PageFlags, Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmSpace},
};

// pub fn create_root_task_space() -> UserSpace {
//     let program = initramfs();
//     let user_pages = {
//         let nframes = program.len().align_up(PAGE_SIZE) / PAGE_SIZE;
//         let vm_frames = VmAllocOptions::new(nframes).alloc().unwrap();
//         // Phyiscal memory pages can be only accessed
//         // via the VmFrame abstraction.
//         vm_frames.write_bytes(0, program).unwrap();
//         vm_frames
//     };
//     let user_address_space = {
//         const MAP_ADDR: Vaddr = 0x0040_0000; // The map addr for statically-linked executable

//         // The page table of the user space can be
//         // created and manipulated safely through
//         // the VmSpace abstraction.
//         let vm_space = VmSpace::new();
//         let mut options = VmMapOptions::new();
//         options.addr(Some(MAP_ADDR)).flags(PageFlags::RWX);
//         vm_space.map(user_pages, &options).unwrap();
//         vm_space
//     };
//     let opt = user_address_space.query(0x521090).unwrap();
//     trace!(" ans is {:?}", opt);
//     let elf = Elf::parse_elf(program).unwrap();
//     // let x = user_address_space.query(0x410600).unwrap();
//     // trace!("{:?}", x);
//     // let x = user_address_space.query(0x420060).unwrap();
//     // trace!("{:?}", x);
//     let user_cpu_state = {
//         // The user-space CPU states can be initialized
//         // to arbitrary values via the UserContext
//         // abstraction.
//         let mut user_cpu_state = UserContext::default();
//         user_cpu_state.set_rip(elf.entry_point());
//         user_cpu_state
//     };
//     UserSpace::new(Arc::new(user_address_space), user_cpu_state)
// }
