use core::fmt::Debug;

mod handle;
mod rights;

/// Common interface of a kernel object
pub trait KernelObject: Debug {
    /// Get object's KoId
    fn tag(&self) -> KTag;
}

pub struct KObjectBase {
    pub tag: KTag,
    inner: KObjectBaseInner,
}

pub struct KObjectBaseInner {}

pub enum KTag {
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


/* 
capability table entry
struct cte {
  cap_t cap;
  mdb_not_t cteMDBNode;
}; 

struct mdb_node {
  uint64_t words[2];
}
typedef struct mdb_node mdb_node_t;



*/