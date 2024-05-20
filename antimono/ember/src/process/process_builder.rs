use alloc::{ffi::CString, sync::Arc, vec::Vec};

use super::{
    process_vm::ProcessVm,
    sel4_thread::{sel4_thread_ext::SeL4ThreadExt, SeL4ThreadBuilder},
    Pid, Process,
};
use crate::{thread::Thread, Result};

pub struct ProcessBuilder<'a> {
    // Essential parts
    pid: Pid,
    elf_binary: &'a [u8],

    // Optional parts
    main_thread_builder: Option<SeL4ThreadBuilder>,
    argv: Option<Vec<CString>>,
    envp: Option<Vec<CString>>,
    process_vm: Option<ProcessVm>,
    is_root_server: bool,
}

impl<'a> ProcessBuilder<'a> {
    pub fn new(pid: Pid, elf_binary: &'a [u8], is_root_server: bool) -> Self {
        ProcessBuilder {
            pid,
            elf_binary,
            main_thread_builder: None,
            argv: None,
            envp: None,
            process_vm: None,
            is_root_server,
        }
    }

    pub fn main_thread_builder(&mut self, builder: SeL4ThreadBuilder) -> &mut Self {
        self.main_thread_builder = Some(builder);
        self
    }

    pub fn process_vm(&mut self, process_vm: ProcessVm) -> &mut Self {
        self.process_vm = Some(process_vm);
        self
    }

    pub fn argv(&mut self, argv: Vec<CString>) -> &mut Self {
        self.argv = Some(argv);
        self
    }

    pub fn envp(&mut self, envp: Vec<CString>) -> &mut Self {
        self.envp = Some(envp);
        self
    }

    pub fn is_root_server(&mut self, is_root_server: bool) -> &mut Self {
        self.is_root_server = is_root_server;
        self
    }

    fn check_build(&self) -> Result<()> {
        // if self.main_thread_builder.is_some() {
        //     debug_assert!(self.parent.upgrade().is_some());
        //     debug_assert!(self.argv.is_none());
        //     debug_assert!(self.envp.is_none());
        //     debug_assert!(self.credentials.is_none());
        // }

        // if self.main_thread_builder.is_none() {
        //     debug_assert!(self.parent.upgrade().is_none());
        //     debug_assert!(self.argv.is_some());
        //     debug_assert!(self.envp.is_some());
        //     debug_assert!(self.credentials.is_some());
        // }

        Ok(())
    }

    pub fn build(self) -> Result<Arc<Process>> {
        self.check_build()?;
        let Self {
            pid,
            elf_binary,
            main_thread_builder,
            argv,
            envp,
            process_vm,
            is_root_server,
        } = self;

        let process_vm = process_vm
            .or_else(|| Some(ProcessVm::alloc(is_root_server)))
            .unwrap();

        let process = {
            let threads = Vec::new();
            Arc::new(Process::new(pid, process_vm, threads))
        };

        let thread = if let Some(thread_builder) = main_thread_builder {
            let builder = thread_builder.process(Arc::downgrade(&process));
            builder.build()
        } else {
            Thread::new_sel4_thread_from_binary(
                pid,
                process.vm(),
                elf_binary,
                argv.unwrap(),
                envp.unwrap(),
                is_root_server,
                Arc::downgrade(&process),
            )?
        };

        process.threads().lock().push(thread);

        process.set_runnable();

        Ok(process)
    }
}
