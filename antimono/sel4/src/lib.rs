#![no_std]

extern crate alloc;
use alloc::sync::Arc;
use aster_frame::{prelude::*, sync::Mutex};

use downcast_rs::{impl_downcast, Downcast};

mod object {
    pub enum ObjectType {
        Untyped,
        Notification,
        CNode,
        Tcb,
        SchedContext,
        Reply,
        // Arch(ObjectTypeArch),
    }
}

#[repr(u32)]
pub enum CapTag {
    Null = 0,
}

type CapRef = Arc<dyn Capability>;

struct CapBase<R: From<u64>> {
    cap_ref: Option<CapRef>,
    rights: R,
}

impl<R: From<u64>> CapBase<R> {
    fn new(cap_ref: Option<CapRef>, rights: R) -> Self {
        Self {
            cap_ref,
            rights,
        }
    }
}

struct NullCap(CapBase<u64>);

impl NullCap {
    fn new() -> Self {
        NullCap(CapBase::new(None, 0))
    }
}

impl Capability for NullCap {
    fn name(&self) -> &'static str {
        "NullCap"
    }

    fn tag(&self) -> CapTag {
        CapTag::Null
    }
}

pub trait Capability: Downcast {
    fn name(&self) -> &'static str;

    fn tag(&self) -> CapTag;

    fn set_cptr(&mut self) {}

    fn set_rights(&mut self, rights: u64) {}
}

impl_downcast!(Capability);

mod error;
