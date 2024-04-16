use alloc::sync::Arc;

use super::{rights::Rights, KernelObject};

/// seL4 capabilities
/// origin is 16B
/// this is 24B
pub struct Handle {
    pub object: Arc<dyn KernelObject>,
    pub rights: Rights,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    extern crate std;

    #[test]
    fn size_test() {
        let sz = mem::size_of::<Arc<dyn KernelObject>>();
        std::println!("{sz}")
    }
}
