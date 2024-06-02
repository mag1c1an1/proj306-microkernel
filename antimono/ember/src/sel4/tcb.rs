use crate::bit;
use crate::sel4::{seL4_SlotBits, seL4_TCBBits};

pub const TCB_SIZE_BITS: usize = seL4_TCBBits as usize - 1usize;
pub const TCB_CNODE_RADIX: usize = 4;
pub const TCB_CNODE_SIZE_BITS: usize = TCB_CNODE_RADIX + seL4_SlotBits as usize;
pub const TCB_OFFSET: usize = bit!(TCB_SIZE_BITS);


// index

pub const TCB_CTABLE: usize = 0;
pub const TCB_VTABLE: usize = 1;
pub const TCB_REPLY: usize = 2;
pub const TCB_CALLER: usize = 3;
pub const TCB_BUFFER: usize = 4;
pub const TCB_CNODE_ENTRIES: usize = 5;
