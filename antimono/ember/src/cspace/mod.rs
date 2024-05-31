use core::marker::PhantomData;

use capability::Capability;

mod capability;
mod raw;

/// a cnode object
pub struct CNode<const SIZE: usize> {
    _phantom_data: PhantomData<[u32; SIZE]>,
}

pub struct Slot {
    // inner:
}

#[cfg(ktest)]
mod test {
    use aster_frame::early_print;
    use ktest::ktest;
}
