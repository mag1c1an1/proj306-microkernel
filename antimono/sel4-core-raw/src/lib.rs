#![no_std]
#![allow(clippy::upper_case_acronyms)]
extern crate alloc;

// use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec;

use align_ext::AlignExt;
use aster_frame::cpu::UserContext;
use aster_frame::prelude::*;
use aster_frame::task::{schedule, Task, TaskOptions};
use aster_frame::user::{UserEvent, UserMode, UserSpace};
use aster_frame::vm::{Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmPerm, VmSpace, PAGE_SIZE};

pub mod object;
pub mod structures;

pub mod config;

pub mod machine;
pub mod model;
pub mod syscall;

pub mod error {
    #[repr(u32)]
    pub enum Exception {
        None = 0,
        Fault = 1,
        Lookup,
        SysCall,
        Preempted,
    }

    pub type Result<T, E = Exception> = core::result::Result<T, E>;
}

pub fn create_user_space(program: &[u8]) -> UserSpace {
    let user_pages = {
        let nframes = program.len().align_up(PAGE_SIZE) / PAGE_SIZE;
        let vm_frames = VmAllocOptions::new(nframes).alloc().unwrap();
        // Phyiscal memory pages can be only accessed
        // via the VmFrame abstraction.
        vm_frames.write_bytes(0, program).unwrap();
        vm_frames
    };
    let user_address_space = {
        const MAP_ADDR: Vaddr = 0x0040_0000; // The map addr for statically-linked executable

        // The page table of the user space can be
        // created and manipulated safely through
        // the VmSpace abstraction.
        let vm_space = VmSpace::new();
        let mut options = VmMapOptions::new();
        options.addr(Some(MAP_ADDR)).perm(VmPerm::RWX);
        vm_space.map(user_pages, &options).unwrap();
        vm_space
    };
    let user_cpu_state = {
        const ENTRY_POINT: Vaddr = 0x0040_1000; // The entry point for statically-linked executable

        // The user-space CPU states can be initialized
        // to arbitrary values via the UserContext
        // abstraction.
        let mut user_cpu_state = UserContext::default();
        user_cpu_state.set_rip(ENTRY_POINT);
        user_cpu_state
    };
    UserSpace::new(user_address_space, user_cpu_state)
}

pub fn create_user_task(user_space: Arc<UserSpace>) -> Arc<Task> {
    fn user_task() {
        let current = Task::current();
        // Switching between user-kernel space is
        // performed via the UserMode abstraction.
        let mut user_mode = {
            let user_space = current.user_space().unwrap();
            UserMode::new(user_space)
        };

        loop {
            // The execute method returns when system
            // calls or CPU exceptions occur.
            let user_event = user_mode.execute();
            // The CPU registers of the user space
            // can be accessed and manipulated via
            // the `UserContext` abstraction.
            let user_context = user_mode.context_mut();
            if UserEvent::Syscall == user_event {
                syscall::handle_syscall(user_context, current.user_space().unwrap());
            }
        }
    }

    // Kernel tasks are managed by the Framework,
    // while scheduling algorithms for them can be
    // determined by the users of the Framework.
    TaskOptions::new(user_task)
        .user_space(Some(user_space))
        .data(0)
        .build()
        .unwrap()
}

pub fn boot_sys() {
    // try boot sys mbi2
    // try_boot_sys();
    // schedule();
    // activateThread();
}

pub fn try_boot_sys() {
    // print kernel load address
    // pic irqs
    // load user img
    /* calculate final location of userland images */
    // and print
    // boot with cpu 0
}

pub fn try_boot_sys_node() {
    // map io
    // set current vspace root
    // skim window
    // init cpu
    // init sys state  ndks
    // init syscall msrs (fast syscalls)
}

pub const LOGO: &str = "
 █████╗ ███╗   ██╗████████╗██╗███╗   ███╗ ██████╗ ███╗   ██╗ ██████╗ 
██╔══██╗████╗  ██║╚══██╔══╝██║████╗ ████║██╔═══██╗████╗  ██║██╔═══██╗
███████║██╔██╗ ██║   ██║   ██║██╔████╔██║██║   ██║██╔██╗ ██║██║   ██║
██╔══██║██║╚██╗██║   ██║   ██║██║╚██╔╝██║██║   ██║██║╚██╗██║██║   ██║
██║  ██║██║ ╚████║   ██║   ██║██║ ╚═╝ ██║╚██████╔╝██║ ╚████║╚██████╔╝
╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ 
";

pub fn init() {
    println!("{}",LOGO)
}
