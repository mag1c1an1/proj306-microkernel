pub use sel4::CNodeCapData;
use sel4_bitfield_ops::Bitfield;

pub mod region;
pub type SeL4Bitfield<T, const N: usize> = Bitfield<[T; N], T>;

/// This is strong Arch related.
/// Pointer in x86_64 pml4 is 48bit, and in sel4 it was stored directly.
/// If the 48th bit is 1, then sign extend.
pub trait PtrSignedExt {
    const PTR_MASK: usize = 1 << 47;
    const SIGNED_MASK: usize = 0xffff << 48;
    fn sign_extend(self) -> Self;
}

impl PtrSignedExt for usize {
    fn sign_extend(self) -> Self {
        if self & Self::PTR_MASK > 0 {
            Self::SIGNED_MASK | self
        } else {
            self
        }
    }
}

#[cfg(ktest)]
mod test {
    use ktest::ktest;

    use crate::common::PtrSignedExt;

    #[ktest]
    fn signed_test() {
        let val = 0usize;
        assert_eq!(val.sign_extend(), 0);
        let val = 1usize << 47;
        assert_eq!(val.sign_extend(), 0xffff_8000_0000_0000);
        let val = 0x8000_ffff_ffffusize;
        assert_eq!(val.sign_extend(), 0xffff_8000_ffff_ffff);
    }
}
