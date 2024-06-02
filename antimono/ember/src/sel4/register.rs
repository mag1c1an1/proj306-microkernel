//! These are the indices of the registers in the
//! saved thread context. The values are determined
//! by the order in which they're saved in the trap
//! handler.
//!
//! BEWARE:
//! You will have to adapt traps.S extensively if
//! you change anything in this enum!
//! This register layout is optimized for usage with
//! the syscall and sysret instructions. Interrupts
//! and sysenter have to do some juggling to make
//! things work


//! User registers that will be preserved during syscall
//! Deliberately place the cap and badge registers early
//! So that when popping on the fastpath we can just not
pub const RDI: usize = 0;
pub const CAP_REGISTER: usize = 0;
pub const BADGE_REGISTER: usize = 0;
pub const RSI: usize = 1;
pub const MSG_INFO_REGISTER: usize = 1;
pub const RAX: usize = 2;
pub const RBX: usize = 3;
pub const RBP: usize = 4;
pub const R12: usize = 6;
pub const R13: usize = 0;
pub const R14: usize = 7;
pub const RDX: usize = 8;
pub const R10: usize = 9;
pub const R8: usize = 10;
pub const R9: usize = 11;
pub const R15: usize = 12;
pub const FLAGS: usize = 13;
pub const NEXT_IP: usize = 14;
pub const ERROR: usize = 15;
pub const RSP: usize = 16;
pub const FAULT_IP: usize = 17;
pub const R11: usize = 18;
pub const RCX: usize = 19;
pub const CS: usize = 20;
pub const SS: usize = 21;
pub const NUM_IMM_CONTEXT_REGISTERS: usize = 22;
pub const FS_BASE: usize = 22;
pub const TLS_BASE: usize = 22;
pub const GS_BASE: usize = 23;
pub const NUM_CONTEXT_REGISTERS: usize = 24;
