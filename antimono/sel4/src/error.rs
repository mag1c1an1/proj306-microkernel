use core::{fmt, result};


pub type Result<T> = result::Result<T, Error>;


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidArgument,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "seL4_Error: {:?}", self)
    }
}