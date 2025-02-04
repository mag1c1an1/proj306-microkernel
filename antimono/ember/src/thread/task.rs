use alloc::sync::{Arc, Weak};
use anti_frame::{
    cpu::UserContext,
    task::{Task, TaskOptions},
    user::{UserContextApi, UserEvent, UserMode, UserSpace},
};

use crate::{current_thread, syscall::handle_syscall};

use super::{exception::handle_exception, Thread};

pub fn create_new_user_task(user_space: Arc<UserSpace>, thread_ref: Weak<Thread>) -> Arc<Task> {
    fn user_task_entry() {
        let current_thread = current_thread!();
        let current_task = Task::current();
        let user_space = current_task
            .user_space()
            .expect("user task should have user space");
        let mut user_mode = UserMode::new(user_space);
        debug!(
            "[Task entry] rip = 0x{:x}",
            user_mode.context().instruction_pointer()
        );
        debug!(
            "[Task entry] rsp = 0x{:x}",
            user_mode.context().stack_pointer()
        );
        debug!(
            "[Task entry] rax = 0x{:x}",
            user_mode.context().syscall_ret()
        );

        loop {
            let user_event = user_mode.execute();
            let context = user_mode.context_mut();
            // handle user event:
            handle_user_event(user_event, context);
            // should be do this comparison before handle signal?
            if current_thread.status().is_exited() {
                break;
            }
            // handle_pending_signal(context, &current_thread).unwrap();
            // if current_thread.status().is_exited() {
            //     debug!("exit due to signal");
            //     break;
            // }
            // If current is suspended, wait for a signal to wake up self
            // while current_thread.status().is_stopped() {
            //     Thread::yield_now();
            //     debug!("{} is suspended.", current_thread.tid());
            //     // handle_pending_signal(context, &current_thread).unwrap();
            // }
            // a preemption point after handling user event.
            // preempt(current_task);
        }
        debug!("exit user loop");
        // FIXME: This is a work around: exit in kernel task entry may be not called. Why this will happen?
        current_task.exit();
    }

    TaskOptions::new(user_task_entry)
        .data(thread_ref)
        .user_space(Some(user_space))
        .build()
        .unwrap()
}

fn handle_user_event(user_event: UserEvent, context: &mut UserContext) {
    match user_event {
        UserEvent::Syscall => handle_syscall(context),
        UserEvent::Exception => handle_exception(context),
    }
}
