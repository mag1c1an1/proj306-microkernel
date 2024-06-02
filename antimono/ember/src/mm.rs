use alloc::sync::Arc;

use crate::bit;
use crate::object::{KernelObject, ObjectType};

#[derive(Debug)]
pub struct UntypedObject<const SIZE_BITS: usize>
    where
        [(); bit!(SIZE_BITS)]:
{
    mem: Arc<[u8; bit!(SIZE_BITS)]>,
}

impl<const SIZE_BITS: usize> UntypedObject<SIZE_BITS>
    where
        [(); bit!(SIZE_BITS)]:
{}


impl<const SIZE_BITS: usize> KernelObject for UntypedObject<SIZE_BITS>
    where
        [(); bit!(SIZE_BITS)]:
{
    fn obj_type(&self) -> ObjectType {
        ObjectType::Untyped
    }

    fn size_bits(&self) -> usize {
        SIZE_BITS
    }
}