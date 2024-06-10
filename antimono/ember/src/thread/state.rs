use core::sync::atomic::{AtomicU8, Ordering};

use num_enum::TryFromPrimitive;

#[derive(Debug)]
pub struct AtomicThreadState(AtomicU8);

impl AtomicThreadState {
    /// Creates a new atomic status.
    pub fn new(status: ThreadState) -> Self {
        Self(AtomicU8::new(status as u8))
    }

    /// Loads a value from the atomic status.
    pub fn load(&self, order: Ordering) -> ThreadState {
        ThreadState::try_from(self.0.load(order)).unwrap()
    }

    /// Stores a value into the atomic status.
    pub fn store(&self, new_status: ThreadState, order: Ordering) {
        self.0.store(new_status as u8, order);
    }

    /// Stores a value into the atomic status if the current value is the same as the `current` value.
    pub fn compare_exchange(
        &self,
        current: ThreadState,
        new: ThreadState,
        success: Ordering,
        failure: Ordering,
    ) -> Result<ThreadState, ThreadState> {
        self.0
            .compare_exchange(current as u8, new as u8, success, failure)
            .map(|val| ThreadState::try_from(val).unwrap())
            .map_err(|val| ThreadState::try_from(val).unwrap())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ThreadState {
    Inactive = 0,
    Running = 1,
    Restart = 2,
    BlockedOnReceive = 3,
    BlockedOnSend = 4,
    BlockedOnReply = 5,
    BlockedOnNotification = 6,
    Idle = 7,
    Exited = 8,
}

impl ThreadState {
    pub fn is_running(&self) -> bool {
        *self == ThreadState::Running
    }

    pub fn is_exited(&self) -> bool {
        *self == ThreadState::Exited
    }

    pub fn is_inactive(&self) -> bool {
        *self == ThreadState::Inactive
    }
    pub fn is_restart(&self) -> bool {
        *self == ThreadState::Restart
    }

    pub fn is_blocked_on_receive(&self) -> bool {
        *self == ThreadState::BlockedOnReceive

    }
    pub fn is_blocked_on_send(&self) -> bool {
        *self == ThreadState::BlockedOnSend
    }
    pub fn is_blocked_on_reply(&self) -> bool {
        *self == ThreadState::BlockedOnReply
    }
    pub fn is_blocked_on_notification(&self) -> bool {
        *self == ThreadState::BlockedOnNotification
    }
    pub fn is_idle(&self) -> bool {
        *self == ThreadState::Idle
    }
}
