use alloc::boxed::Box;

use aster_frame::{
    sync::SpinLock,
    task::{Scheduler, set_scheduler, TaskAdapter},
};
use aster_frame::task::Task;
use intrusive_collections::LinkedList;

pub fn init() {
    let sel4_scheduler = Box::new(SeL4Scheduler::new());
    let scheduler = Box::leak(sel4_scheduler);
    set_scheduler(scheduler);
}

struct SeL4Scheduler {
    tasks: SpinLock<LinkedList<TaskAdapter>>,
}

impl SeL4Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: SpinLock::new(LinkedList::new(TaskAdapter::new())),
        }
    }
}

impl Scheduler for SeL4Scheduler {
    fn enqueue(&self, task: alloc::sync::Arc<Task>) {
        self.tasks.lock_irq_disabled().push_back(task);
    }

    fn dequeue(&self) -> Option<alloc::sync::Arc<Task>> {
        self.tasks.lock_irq_disabled().pop_front()
    }

    fn should_preempt(&self, _task: &alloc::sync::Arc<Task>) -> bool {
        false
    }
}
