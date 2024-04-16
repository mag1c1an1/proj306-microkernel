#![no_std]

extern crate alloc;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Capability
#[derive(Debug, Clone, Copy, Default)]
pub struct Cap {
    pub word: [u64; 2],
}

impl Cap {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn null_cap() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CapTag {
    Null = 0,
    Untyped = 2,
    Endpoint = 4,
    Notification = 6,
    Reply = 8,
    Cnode = 10,
    Thread = 12,
    IrqControl = 14,
    IrqHandler = 16,
    Zombie = 18,
    Domain = 20,
    Frame = 1,
    PageTable = 3,
    PageDirectory = 5,
    Pdpt = 7,
    Pml4 = 9,
    AsidControl = 11,
    AsidPool = 13,
    IoPort = 19,
    IoPortControl = 31,
}


pub mod syscall {
    fn debug_putchar(c:char) {todo!()}
}


pub mod object;


// pub mod object {


    






//     mod untyped {
//         // alignup
//         // decodeUntypedInvocation
//         // resetUntypedCap
//         // invokeUntyped_Retype
//     }
//     mod cnode {}
// }

mod error;
pub use error::*;