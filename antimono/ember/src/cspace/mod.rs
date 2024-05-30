use cap::{Capability, RawCap};
mod cap;
pub struct CNode<const SIZE: usize> {
    slots: [u32; SIZE],
}

#[cfg(ktest)]
mod test {
    use aster_frame::early_print;
    use ktest::ktest;
}


fn tmp() {
   core::ptr::Pointee
}