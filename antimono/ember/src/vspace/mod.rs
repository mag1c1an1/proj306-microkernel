use core::marker::PhantomData;

use aster_frame::vm::VmFrameVec;

#[derive(Debug)]
pub struct MemType<T> {
    frames: VmFrameVec,
    _marker: PhantomData<T>,
}

impl<T> MemType<T> {
    pub fn new(frames: VmFrameVec) -> Self {
        Self { frames, _marker: Default::default() }
    }
}