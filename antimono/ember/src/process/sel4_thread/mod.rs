use core::ffi::CStr;

use alloc::{
    ffi::CString,
    sync::{Arc, Weak},
};
use anti_frame::{sync::Mutex, user::UserSpace};

use super::Process;
use crate::{
    error::{Errno, Error},
    thread::{status::ThreadStatus, task, thread_table, Thread, Tid},
    Result,
};

pub mod sel4_thread_ext;

#[derive(Debug)]
pub struct SeL4Thread {
    process: Weak<Process>,
    is_root_server: bool,
    name: Mutex<Option<ThreadName>>,
    // capabilities
}

impl SeL4Thread {
    pub fn process(&self) -> Arc<Process> {
        self.process.upgrade().unwrap()
    }
    pub fn thread_name(&self) -> &Mutex<Option<ThreadName>> {
        &self.name
    }
}

pub const MAX_THREAD_NAME_LEN: usize = 16;

#[derive(Debug)]
pub struct ThreadName {
    inner: [u8; MAX_THREAD_NAME_LEN],
    count: usize,
}

impl ThreadName {
    pub fn new() -> Self {
        ThreadName {
            inner: [0; MAX_THREAD_NAME_LEN],
            count: 0,
        }
    }

    pub fn new_from_executable_path(executable_path: &str) -> Result<Self> {
        let mut thread_name = ThreadName::new();
        let executable_file_name = executable_path
            .split('/')
            .last()
            .ok_or(Error::with_message(Errno::EINVAL, "invalid elf path"))?;
        let name = CString::new(executable_file_name)?;
        thread_name.set_name(&name)?;
        Ok(thread_name)
    }

    pub fn set_name(&mut self, name: &CStr) -> Result<()> {
        let bytes = name.to_bytes_with_nul();
        let bytes_len = bytes.len();
        if bytes_len > MAX_THREAD_NAME_LEN {
            // if len > MAX_THREAD_NAME_LEN, truncate it.
            self.count = MAX_THREAD_NAME_LEN;
            self.inner[..MAX_THREAD_NAME_LEN].clone_from_slice(&bytes[..MAX_THREAD_NAME_LEN]);
            self.inner[MAX_THREAD_NAME_LEN - 1] = 0;
            return Ok(());
        }
        self.count = bytes_len;
        self.inner[..bytes_len].clone_from_slice(bytes);
        Ok(())
    }

    pub fn name(&self) -> Result<Option<&CStr>> {
        Ok(Some(CStr::from_bytes_until_nul(&self.inner)?))
    }
}

/// The builder to build a sel4 thread
pub struct SeL4ThreadBuilder {
    // The essential part
    tid: Tid,
    user_space: Arc<UserSpace>,
    process: Weak<Process>,

    // Optional part
    thread_name: Option<ThreadName>,
    is_root_server: bool,
}

impl SeL4ThreadBuilder {
    pub fn new(tid: Tid, user_space: Arc<UserSpace>, is_root_server: bool) -> Self {
        Self {
            tid,
            user_space,
            process: Weak::new(),
            thread_name: None,
            is_root_server,
        }
    }

    pub fn process(mut self, process: Weak<Process>) -> Self {
        self.process = process;
        self
    }

    pub fn thread_name(mut self, thread_name: Option<ThreadName>) -> Self {
        self.thread_name = thread_name;
        self
    }

    pub fn is_main_thread(mut self, is_root_server: bool) -> Self {
        self.is_root_server = is_root_server;
        self
    }

    pub fn build(self) -> Arc<Thread> {
        let Self {
            tid,
            user_space,
            process,
            thread_name,
            is_root_server,
        } = self;

        // let real_timer = RealTimer::new(move || {
        //     let process = {
        //         let Some(current_thread) = thread_table::get_thread(tid) else {
        //             return;
        //         };
        //         let posix_thread = current_thread.as_posix_thread().unwrap();
        //         posix_thread.process()
        //     };

        //     let signal = KernelSignal::new(SIGALRM);
        //     process.enqueue_signal(signal);
        // })
        // .unwrap();

        let thread = Arc::new_cyclic(|thread_ref| {
            let task = task::create_new_user_task(user_space, thread_ref.clone());
            let status = ThreadStatus::Init;
            let posix_thread = SeL4Thread {
                process,
                is_root_server,
                name: Mutex::new(thread_name),
            };

            Thread::new(tid, task, posix_thread, status)
        });
        thread_table::add_thread(thread.clone());
        thread
    }
}
