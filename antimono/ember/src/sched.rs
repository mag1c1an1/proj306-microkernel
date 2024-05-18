use alloc::boxed::Box;
use anti_frame::{
    sync::SpinLock,
    task::{set_scheduler, Scheduler, TaskAdapter},
};
use intrusive_collections::LinkedList;
pub fn init() {
    let sel4_scheduler = Box::new(SeL4Schduler::new());
    let scheduler = Box::leak(sel4_scheduler);
    set_scheduler(scheduler);
}

struct SeL4Schduler {
    tasks: SpinLock<LinkedList<TaskAdapter>>,
}

impl SeL4Schduler {
    pub fn new() -> Self {
        Self {
            tasks: SpinLock::new(LinkedList::new(TaskAdapter::new())),
        }
    }
}

impl Scheduler for SeL4Schduler {
    fn enqueue(&self, task: alloc::sync::Arc<anti_frame::task::Task>) {
        self.tasks.lock_irq_disabled().push_back(task);
    }

    fn dequeue(&self) -> Option<alloc::sync::Arc<anti_frame::task::Task>> {
        self.tasks.lock_irq_disabled().pop_front()
    }

    fn should_preempt(&self, _task: &alloc::sync::Arc<anti_frame::task::Task>) -> bool {
        false
    }
}
