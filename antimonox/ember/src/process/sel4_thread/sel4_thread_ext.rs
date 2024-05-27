use core::mem;

use alloc::{
    boxed::Box,
    ffi::CString,
    sync::{Arc, Weak},
    vec::Vec,
};
use anti_frame::{
    cpu::UserContext,
    user::UserSpace,
    vm::{Vaddr, VmIo},
};
use anti_rights::{Full, FullOp, Rights, Write};

use crate::{
    process::{
        process_vm::{ProcessVm, USER_HEAP_BASE},
        program_loader::load_program_to_vm,
        Process,
    },
    sel4::{boot::bootstrap::init_rootserver, CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS},
    thread::{Thread, Tid},
    vm::{
        perms::VmPerms,
        vmo::{VmoOptions, VmoRightsOp},
    },
    Result,
};

use super::{SeL4Thread, SeL4ThreadBuilder};

pub trait SeL4ThreadExt {
    fn as_sel4_thread(&self) -> Option<&SeL4Thread>;
    fn new_sel4_thread_from_binary(
        tid: Tid,
        process_vm: &ProcessVm,
        elf_binary: &[u8],
        argv: Vec<CString>,
        envp: Vec<CString>,
        is_root_server: bool,
        process: Weak<Process>,
    ) -> Result<Arc<Self>>;
}

impl SeL4ThreadExt for Thread {
    fn as_sel4_thread(&self) -> Option<&SeL4Thread> {
        self.data().downcast_ref::<SeL4Thread>()
    }

    fn new_sel4_thread_from_binary(
        tid: Tid,
        process_vm: &ProcessVm,
        elf_binary: &[u8],
        argv: Vec<CString>,
        envp: Vec<CString>,
        is_root_server: bool,
        process: Weak<Process>,
    ) -> Result<Arc<Self>> {
        let elf_load_info = load_program_to_vm(process_vm, elf_binary, argv, envp, is_root_server)?;
        let vptr = init_rootserver(process_vm);
        // add sel4 root_server boot_info
        // TODO change this
        // let vmo = create_rootserver_vmo();
        // let slot_region = seL4_SlotRegion { start: 0, end: 0 };
        // let info = seL4_BootInfo {
        //     extraLen: 10,
        //     nodeID: 0,
        //     numNodes: 1,
        //     numIOPTLevels: 2,
        //     ipcBuffer: 0,
        //     empty: slot_region,
        //     sharedFrames: slot_region,
        //     userImageFrames: slot_region,
        //     userImagePaging: slot_region,
        //     ioSpaceCaps: slot_region,
        //     extraBIPages: slot_region,
        //     initThreadCNodeSizeBits: 100,
        //     initThreadDomain: 1,
        //     untyped: slot_region,
        //     untypedList: [seL4_UntypedDesc {
        //         paddr: 0,
        //         sizeBits: 0,
        //         isDevice: 0,
        //         padding: [0; 6],
        //     }; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
        // };
        // let root_vmar = process_vm.root_vmar();
        // let map_addr = root_vmar
        //     .new_map(vmo, VmPerms::READ | VmPerms::WRITE)?
        //     .build()?;
        trace!("boot info vaddr 0x{:x}", vptr);
        let mut cpu_ctx = UserContext::default();
        let vm_space = process_vm.root_vmar().vm_space().clone();
        cpu_ctx.set_rip(elf_load_info.entry_point() as _);
        cpu_ctx.set_rdi(vptr);
        if let Some(statck_top) = elf_load_info.user_stack_top() {
            cpu_ctx.set_rsp(statck_top as _);
        }
        let user_space = Arc::new(UserSpace::new(vm_space, cpu_ctx));
        // let thread_name = Some(ThreadName::new_from_executable_path(executable_path)?);
        let thread_builder =
            SeL4ThreadBuilder::new(tid, user_space, is_root_server).process(process);
        Ok(thread_builder.build())
    }
}

//
fn init_initial_thread(
    tid: Tid,
    process_vm: &ProcessVm,
    elf_binary: &[u8],
    process: Weak<Process>,
) {
    // first compute the memory size
}
