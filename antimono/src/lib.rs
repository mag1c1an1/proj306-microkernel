#![no_std]
// #![forbid(unsafe_code)]

// extern crate alloc;
use anti_frame::prelude::*;
// use aster_frame::sync::Mutex;

// use alloc::boxed::Box;
// use alloc::collections::VecDeque;
// use alloc::sync::Arc;
// use alloc::vec::{self, Vec};
// use aster_frame::task::Scheduler;

#[aster_main]
fn kernel_main() {
    // println!("[antimono] finish init aster frame");
    // let x:Box<dyn Scheduler> = Box::new(Sched::new());
    // let s:&'static mut dyn Scheduler = Box::leak(x);
    // aster_frame::task::set_scheduler(s);
    sel4_core_raw::init();
    // // init other things
    // let program_binary = include_bytes!("../rootserver/hello-bin");
    // let user_space = sel4::create_user_space(program_binary);
    // let user_task = sel4::create_user_task(Arc::new(user_space));
    // user_task.run();
}

// struct Sched {
//     queue: Mutex<VecDeque<Arc<aster_frame::task::Task>>>,
// }

// impl Sched {
//     fn new() -> Self {
//         Sched {
//             queue: Mutex::new(VecDeque::new()),
//         }
//     }
// }

// impl aster_frame::task::Scheduler for Sched {
//     fn enqueue(&self, task: Arc<aster_frame::task::Task>) {
//         self.queue.lock().push_back(task);
//     }

//     fn dequeue(&self) -> Option<Arc<aster_frame::task::Task>> {
//         log::error!("dequeue");
//         self.queue.lock().pop_front()
//     }

//     fn should_preempt(&self, task: &Arc<aster_frame::task::Task>) -> bool {
//         false
//     }
// }
