use core::ops::Range;

/// physical address , (paddr_t in sel4)
pub use aster_frame::prelude::Paddr;
/// virtual address for userspace, (vptr_t in sel4)
pub use aster_frame::prelude::Vaddr;

/// kernel virtual address (pptr_t in sel4)
pub type Kaddr = usize;

/// region_t in sel4
pub type Region = Range<Kaddr>;
/// v_region_t in sel4
pub type VRegion = Range<Vaddr>;
/// p_region_t in sel4
pub type PRegion = Range<Paddr>;

mod test {
    use ktest::ktest;

    use crate::common::region::Region;

    #[ktest]
    fn region_test() {
        assert_eq!(16, core::mem::size_of::<Region>());
    }
}