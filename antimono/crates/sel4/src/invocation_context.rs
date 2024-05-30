//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: MIT
//

use core::cell::RefCell;

use crate::IpcBuffer;

/// A strategy for discovering the current thread's IPC buffer.
pub trait InvocationContext {
    fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T;
}

/// The absence of a strategy for discovering the current thread's IPC buffer.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NoInvocationContext;

impl NoInvocationContext {
    pub const fn new() -> Self {
        Self
    }
}

impl InvocationContext for &mut IpcBuffer {
    fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T {
        f(self)
    }
}

impl<U: InvocationContext> InvocationContext for &mut U {
    fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T {
        U::with_context(self, f)
    }
}

impl<U: InvocationContext> InvocationContext for &RefCell<U> {
    fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T {
        U::with_context(&mut self.borrow_mut(), f)
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "state")] {
        type NoExplicitInvocationContextInternal = crate::ImplicitInvocationContext;
    } else {
        type NoExplicitInvocationContextInternal = NoInvocationContext;
    }
}

/// The default strategy for discovering the current thread's IPC buffer.
///
/// When the `"state"` feature is enabled, [`NoExplicitInvocationContext`] is an alias for
/// [`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`]
/// set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
/// [`NoInvocationContext`](crate::NoInvocationContext), which does not implement
/// [`InvocationContext`].
pub type NoExplicitInvocationContext = NoExplicitInvocationContextInternal;
