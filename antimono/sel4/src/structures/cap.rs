/// origin in strcuture_gen.h
#[derive(Debug, Copy, Clone)]
pub(crate) struct Cap {
    words: [u64; 2],
}

#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    PDPT = 7,
    PML4 = 9,
    AsidControl = 11,
    AsidPool = 13,
    IOPort = 19,
    IOPortControl = 31,
}

impl From<u64> for CapTag {
    fn from(value: u64) -> Self {
        match value {
            2 => Self::Untyped,
            4 => Self::Endpoint,
            6 => Self::Notification,
            8 => Self::Reply,
            10 => Self::Cnode,
            12 => Self::Thread,
            14 => Self::IrqControl,
            16 => Self::IrqHandler,
            18 => Self::Zombie,
            20 => Self::Domain,
            1 => Self::Frame,
            3 => Self::PageTable,
            5 => Self::PageDirectory,
            7 => Self::PDPT,
            9 => Self::PML4,
            11 => Self::AsidControl,
            13 => Self::AsidPool,
            19 => Self::IOPort,
            31 => Self::IOPortControl,
            _ => Self::Null, // 0 and others
        }
    }
}

impl From<CapTag> for u64 {
    fn from(value: CapTag) -> Self {
        value as u64
    }
}

// general cap
impl Cap {
    pub fn new_null_cap() -> Self {
        let word0 = 0 | u64::from(CapTag::Null) & 0x1f << 59;
        Self { words: [word0, 0] }
    }

    pub fn typ(&self) -> u64 {
        (self.words[0] >> 59) & 0x1
    }

    pub fn type_equals(&self, cap_type_tag: u64) -> bool {
        self.typ() == cap_type_tag
    }

    /* Returns whether or not this capability has memory associated
     * with it or not. Referring to this as 'being physical' is to
     * match up with the Haskell and abstract specifications */
    pub fn is_physical(&self) -> bool {
        todo!()
    }

    pub fn cap_ptr(&self) -> u64 {
        // TODO
        assert!(self.typ() == CapTag::Untyped.into());
        self.words[0] & 0xffffffffffff
    }

    pub fn updat_data(&mut self, preserve: bool, new_data: u64) {
        todo!()
    }

    pub fn is_same_regioin_as(&self, other: &Self) -> bool {
        todo!()
    }

    pub fn is_same_object_as(&self, other: &Self) -> bool {
        todo!()
    }
}

// untyped
impl Cap {
    pub fn new_untyped_new(free_index: u64, is_device: u64, block_size: u64, cap_ptr: u64) -> Self {
        let word0 = 0 | u64::from(CapTag::Untyped) & 0x1f << 59 | (cap_ptr & 0xffffffffffff);
        let words1 =
            0 | (free_index & 0xffffffffffff) << 16 | (is_device & 0x1) << 6 | (block_size & 0x3f);
        Self {
            words: [word0, words1],
        }
    }

    pub fn free_index(&self) -> u64 {
        assert!(self.typ() == CapTag::Untyped.into());
        self.words[1] & 0xffffffffffff0000 >> 16
    }

    /// should assert free_index is valid
    /// this beyound the ability of u64
    pub fn set_free_index(&mut self, free_index: u64) {
        assert!(self.typ() == CapTag::Untyped.into());
        self.words[1] &= !0xffffffffffff0000;
        self.words[1] |= (free_index << 16) & 0xffffffffffff0000;
    }

    pub fn is_device(&self) -> u64 {
        assert!(self.typ() == CapTag::Untyped.into());
        self.words[1] & 0x40 >> 6
    }

    pub fn block_size(&self) -> u64 {
        assert!(self.typ() == CapTag::Untyped.into());
        self.words[1] & 0x3f
    }

    pub fn size_bits(&self) -> u64 {
        todo!()
    }
}

// derive_cap in rust should be Result<Cap>

pub struct FinaliseCapRet {
    pub remainder: Cap,
    pub cleanup_info: Cap,
}

pub struct CapTransfer {}



#[cfg(test)]
/// TODO add tests
mod tests {}
