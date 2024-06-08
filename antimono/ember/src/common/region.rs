use core::ops::Range;

/// physical address , (paddr_t in sel4)
pub use aster_frame::prelude::Paddr;
/// virtual address for userspace, (vptr_t in sel4)
pub use aster_frame::prelude::Vaddr;

/// kernel virtual address (pptr_t in sel4)
pub type Kaddr = usize;
/// this is aster_frame's offset
const KERNEL_OFFSET:usize = 0xffff_8000_0000_0000;
pub fn paddr_to_kaddr(paddr: Paddr) -> Kaddr {
    paddr + KERNEL_OFFSET
}

pub fn kaddr_to_paddr(kaddr: Kaddr) -> Paddr {
    kaddr - KERNEL_OFFSET
}

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