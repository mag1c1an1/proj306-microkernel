use alloc::{
    collections::{btree_map::Values, BTreeMap},
    ffi::CString,
    string::String,
    sync::Arc,
    vec::Vec,
};
use anti_frame::sync::{Mutex, MutexGuard};
use anti_rights::Full;

use crate::{
    thread::{allocate_tid, Thread},
    vm::vmar::Vmar,
    Result,
};

use self::sel4_thread::sel4_thread_ext::SeL4ThreadExt;
use self::{
    process_builder::ProcessBuilder,
    process_vm::{Heap, InitStackReader, ProcessVm},
};

pub mod process_builder;
pub mod process_vm;
pub mod program_loader;
pub mod sel4_thread;

pub type Pid = u32;

static PROCESS_TABLE: Mutex<BTreeMap<Pid, Arc<Process>>> = Mutex::new(BTreeMap::new());

// ************ Process *************

/// Gets a process with pid
pub fn get_process(pid: &Pid) -> Option<Arc<Process>> {
    PROCESS_TABLE.lock().get(pid).cloned()
}

fn process_table_mut() -> MutexGuard<'static, BTreeMap<Pid, Arc<Process>>> {
    PROCESS_TABLE.lock()
}

/// Acquires a lock on the process table and returns a `ProcessTable`.
pub fn process_table() -> ProcessTable<'static> {
    ProcessTable {
        inner: PROCESS_TABLE.lock(),
    }
}

/// A wrapper for the mutex-protected process table.
///
/// It provides the `iter` method to iterator over the processes in the table.
pub struct ProcessTable<'a> {
    inner: MutexGuard<'a, BTreeMap<Pid, Arc<Process>>>,
}

impl<'a> ProcessTable<'a> {
    /// Returns an iterator over the processes in the table.
    pub fn iter(&self) -> ProcessTableIter {
        ProcessTableIter {
            inner: self.inner.values(),
        }
    }
}

/// An iterator over the processes of the process table.
pub struct ProcessTableIter<'a> {
    inner: Values<'a, Pid, Arc<Process>>,
}

impl<'a> Iterator for ProcessTableIter<'a> {
    type Item = &'a Arc<Process>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Process {
    pid: Pid,
    process_vm: ProcessVm,
    threads: Mutex<Vec<Arc<Thread>>>,
    status: Mutex<ProcessStatus>,
}

impl Process {
    fn new(pid: Pid, process_vm: ProcessVm, threads: Vec<Arc<Thread>>) -> Self {
        Self {
            pid,
            process_vm,
            threads: Mutex::new(threads),
            status: Mutex::new(ProcessStatus::Uninit),
        }
    }

    pub fn spawn_user_process(
        elf_binary: &[u8],
        argv: Vec<CString>,
        envp: Vec<CString>,
        is_root_server: bool,
    ) -> Result<Arc<Self>> {
        let process = Process::create_user_process(elf_binary, argv, envp, is_root_server)?;
        process.run();
        Ok(process)
    }

    fn create_user_process(
        elf_binary: &[u8],
        argv: Vec<CString>,
        envp: Vec<CString>,
        is_root_server: bool,
    ) -> Result<Arc<Self>> {
        let process_builder = {
            let pid = allocate_pid();
            let mut builder = ProcessBuilder::new(pid, elf_binary, is_root_server);
            builder.argv(argv).envp(envp);
            builder
        };
        let process = process_builder.build()?;

        let mut process_table_mut = process_table_mut();

        process_table_mut.insert(process.pid(), process.clone());

        Ok(process)
    }

    pub fn run(&self) {
        let threads = self.threads.lock();
        // when run the process, the process should has only one thread
        debug_assert!(threads.len() == 1);
        debug_assert!(self.is_runnable());
        let thread = threads[0].clone();
        // should not hold the lock when run thread
        drop(threads);
        thread.run();
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn threads(&self) -> &Mutex<Vec<Arc<Thread>>> {
        &self.threads
    }

    pub fn main_thread(&self) -> Option<Arc<Thread>> {
        self.threads
            .lock()
            .iter()
            .find(|thread| thread.tid() == self.pid)
            .cloned()
    }

    // ******* VM ********
    pub fn vm(&self) -> &ProcessVm {
        &self.process_vm
    }

    pub fn root_vmar(&self) -> &Vmar<Full> {
        self.process_vm.root_vmar()
    }

    pub fn init_stack_reader(&self) -> Option<InitStackReader> {
        self.process_vm.init_stack_reader()
    }

    pub fn heap(&self) -> Option<&Heap> {
        self.process_vm.heap()
    }

    // ****** Status *****
    pub fn set_runnable(&self) {
        self.status.lock().set_runnable();
    }

    pub fn is_runnable(&self) -> bool {
        self.status.lock().is_runnable()
    }

    pub fn exit_code(&self) -> Option<u32> {
        // TODO
        None
    }
}

pub fn current() -> Arc<Process> {
    let current_thread = Thread::current();
    if let Some(sel4_thread) = current_thread.as_sel4_thread() {
        sel4_thread.process()
    } else {
        panic!("[Internal error]The current thread does not belong to a process");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    // Not ready to run
    Uninit,
    /// Can be scheduled to run
    Runnable,
}

impl ProcessStatus {
    pub fn set_runnable(&mut self) {
        *self = ProcessStatus::Runnable;
    }

    pub fn is_runnable(&self) -> bool {
        *self == ProcessStatus::Runnable
    }
}

pub fn allocate_pid() -> Pid {
    allocate_tid()
}

/// return current process
#[macro_export]
macro_rules! current {
    () => {
        $crate::process::current()
    };
}
