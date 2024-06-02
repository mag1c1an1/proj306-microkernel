use core::ops::Range;

/// kernel virtual address (pptr_t in sel4)
pub type KAddr = usize;
/// virtual address for userspace, (vptr_t in sel4)
pub type VAddr = usize;
/// physical address , (paddr_t in sel4)
pub type PAddr = usize;


/// region_t in sel4
pub type Region = Range<KAddr>;
/// v_region_t in sel4
pub type VRegion = Range<VAddr>;
/// p_region_t in sel4
pub type PRegion = Range<PAddr>;

mod test {
    use ktest::ktest;

    use crate::common::region::Region;

    #[ktest]
    fn region_test() {
        assert_eq!(16, core::mem::size_of::<Region>());
    }
}