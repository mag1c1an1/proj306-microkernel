#![no_std]

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
