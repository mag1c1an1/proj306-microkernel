// 16 bytes

use sel4::CPtr;

use crate::cspace::raw::{CapType, RawCap};

pub trait Capability {
    /// unpack a raw cap to self
    fn unpack(raw_cap: RawCap) -> Self;
    /// return the type of this cap
    fn typ(&self) -> CapType;
    /// pack self to a sel4 raw cap
    fn pack(&self) -> RawCap;
}

pub struct CnodeCap {
    // cap ptr
    cptr: CPtr,
    radix: usize,
    guard_size: usize,
    guard: usize,
}


mod test {
    #[ktest::ktest]
    fn xx() {}
}
