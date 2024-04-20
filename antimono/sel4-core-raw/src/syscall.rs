use alloc::vec;
use aster_frame::{cpu::UserContext, task::Task, user::UserSpace, vm::VmIo};

pub fn handle_syscall(user_context: &mut UserContext, user_space: &UserSpace) {
    const SYS_WRITE: usize = 1;
    const SYS_EXIT: usize = 60;

    match user_context.rax() {
        SYS_WRITE => {
            // Access the user-space CPU registers safely.
            let (fd, buf_addr, buf_len) =
                (user_context.rdi(), user_context.rsi(), user_context.rdx());
            let buf = {
                let mut buf = vec![0u8; buf_len];
                // Copy data from the user space without
                // unsafe pointer dereferencing.
                user_space
                    .vm_space()
                    .read_bytes(buf_addr, &mut buf)
                    .unwrap();
                  buf
            };
            // Use the console for output safely.
            aster_frame::prelude::println!("{}", core::str::from_utf8(&buf).unwrap());
            // Manipulate the user-space CPU registers safely.
            user_context.set_rax(buf_len);
        }
        SYS_EXIT => Task::current().exit(),
        _ => unimplemented!(),
    }
}