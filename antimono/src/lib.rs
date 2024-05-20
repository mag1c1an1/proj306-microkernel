#![no_std]
// #![forbid(unsafe_code)]

extern crate alloc;
use alloc::{boxed::Box, collections::VecDeque, sync::Arc};
use anti_frame::{prelude::*, sync::Mutex, task::Scheduler};
use log::trace;
// use aster_frame::sync::Mutex;

// use alloc::boxed::Box;
// use alloc::collections::VecDeque;
// use alloc::sync::Arc;
// use alloc::vec::{self, Vec};
// use aster_frame::task::Scheduler;

#[aster_main]
fn kernel_main() {
    // println!("[antimono] finish init aster frame");
    ember::init();
    ember::run_root_server();
}

struct Sched {
    queue: Mutex<VecDeque<Arc<anti_frame::task::Task>>>,
}

impl Sched {
    fn new() -> Self {
        Sched {
            queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl anti_frame::task::Scheduler for Sched {
    fn enqueue(&self, task: Arc<anti_frame::task::Task>) {
        self.queue.lock().push_back(task);
    }

    fn dequeue(&self) -> Option<Arc<anti_frame::task::Task>> {
        log::error!("dequeue");
        self.queue.lock().pop_front()
    }

    fn should_preempt(&self, task: &Arc<anti_frame::task::Task>) -> bool {
        false
    }
}
