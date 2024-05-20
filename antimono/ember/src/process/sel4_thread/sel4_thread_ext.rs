use alloc::{
    ffi::CString,
    sync::{Arc, Weak},
    vec::Vec,
};
use anti_frame::{cpu::UserContext, user::UserSpace};

use crate::{
    process::{process_vm::ProcessVm, program_loader::load_program_to_vm, Process},
    thread::{Thread, Tid},
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
        let mut cpu_ctx = UserContext::default();
        let vm_space = process_vm.root_vmar().vm_space().clone();
        cpu_ctx.set_rip(elf_load_info.entry_point() as _);
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
