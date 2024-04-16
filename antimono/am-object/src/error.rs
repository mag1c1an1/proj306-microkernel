pub type AmResult<T = ()> = Result<T, AmError>;
/// SeL4 errors
#[derive(Debug)]
pub enum AmError {
    NOError = 0,
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
