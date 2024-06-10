use core::fmt;
use crate::common::bit_field::Bitfield;
use crate::common::SeL4Bitfield;

/// copy from rust-sel4 shared types in sel4
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_MessageInfo(pub SeL4Bitfield<u64, 1usize>);
pub type seL4_MessageInfo_t = seL4_MessageInfo;
impl seL4_MessageInfo {
    pub fn new(label: u64, capsUnwrapped: u64, extraCaps: u64, length: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_label(label);
        this.set_capsUnwrapped(capsUnwrapped);
        this.set_extraCaps(extraCaps);
        this.set_length(length);
        this
    }
    pub fn unpack(&self) -> seL4_MessageInfo_Unpacked {
        seL4_MessageInfo_Unpacked {
            label: self.get_label(),
            capsUnwrapped: self.get_capsUnwrapped(),
            extraCaps: self.get_extraCaps(),
            length: self.get_length(),
        }
    }
    #[allow(dead_code)]
    pub fn get_label(&self) -> u64 {
        self.0.get_bits(12usize..64usize)
    }
    pub fn set_label(&mut self, label: u64) {
        self.0.set_bits(12usize..64usize, label)
    }
    #[allow(dead_code)]
    pub const fn width_of_label() -> usize {
        64usize - 12usize
    }
    #[allow(dead_code)]
    pub fn get_capsUnwrapped(&self) -> u64 {
        self.0.get_bits(9usize..12usize)
    }
    pub fn set_capsUnwrapped(&mut self, capsUnwrapped: u64) {
        self.0.set_bits(9usize..12usize, capsUnwrapped)
    }
    #[allow(dead_code)]
    pub const fn width_of_capsUnwrapped() -> usize {
        12usize - 9usize
    }
    #[allow(dead_code)]
    pub fn get_extraCaps(&self) -> u64 {
        self.0.get_bits(7usize..9usize)
    }
    pub fn set_extraCaps(&mut self, extraCaps: u64) {
        self.0.set_bits(7usize..9usize, extraCaps)
    }
    #[allow(dead_code)]
    pub const fn width_of_extraCaps() -> usize {
        9usize - 7usize
    }
    #[allow(dead_code)]
    pub fn get_length(&self) -> u64 {
        self.0.get_bits(0usize..7usize)
    }
    pub fn set_length(&mut self, length: u64) {
        self.0.set_bits(0usize..7usize, length)
    }
    #[allow(dead_code)]
    pub const fn width_of_length() -> usize {
        7usize - 0usize
    }
}
impl fmt::Debug for seL4_MessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_MessageInfo_Unpacked {
    pub label: u64,
    pub capsUnwrapped: u64,
    pub extraCaps: u64,
    pub length: u64,
}
impl seL4_MessageInfo_Unpacked {
    pub fn pack(self) -> seL4_MessageInfo {
        match self {
            Self {
                label,
                capsUnwrapped,
                extraCaps,
                length,
            } => seL4_MessageInfo::new(label, capsUnwrapped, extraCaps, length),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_CapRights(pub SeL4Bitfield<u64, 1usize>);
pub type seL4_CapRights_t = seL4_CapRights;
impl seL4_CapRights {
    pub fn new(
        capAllowGrantReply: u64,
        capAllowGrant: u64,
        capAllowRead: u64,
        capAllowWrite: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capAllowGrantReply(capAllowGrantReply);
        this.set_capAllowGrant(capAllowGrant);
        this.set_capAllowRead(capAllowRead);
        this.set_capAllowWrite(capAllowWrite);
        this
    }
    pub fn unpack(&self) -> seL4_CapRights_Unpacked {
        seL4_CapRights_Unpacked {
            capAllowGrantReply: self.get_capAllowGrantReply(),
            capAllowGrant: self.get_capAllowGrant(),
            capAllowRead: self.get_capAllowRead(),
            capAllowWrite: self.get_capAllowWrite(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capAllowGrantReply(&self) -> u64 {
        self.0.get_bits(3usize..4usize)
    }
    pub fn set_capAllowGrantReply(&mut self, capAllowGrantReply: u64) {
        self.0.set_bits(3usize..4usize, capAllowGrantReply)
    }
    #[allow(dead_code)]
    pub const fn width_of_capAllowGrantReply() -> usize {
        4usize - 3usize
    }
    #[allow(dead_code)]
    pub fn get_capAllowGrant(&self) -> u64 {
        self.0.get_bits(2usize..3usize)
    }
    pub fn set_capAllowGrant(&mut self, capAllowGrant: u64) {
        self.0.set_bits(2usize..3usize, capAllowGrant)
    }
    #[allow(dead_code)]
    pub const fn width_of_capAllowGrant() -> usize {
        3usize - 2usize
    }
    #[allow(dead_code)]
    pub fn get_capAllowRead(&self) -> u64 {
        self.0.get_bits(1usize..2usize)
    }
    pub fn set_capAllowRead(&mut self, capAllowRead: u64) {
        self.0.set_bits(1usize..2usize, capAllowRead)
    }
    #[allow(dead_code)]
    pub const fn width_of_capAllowRead() -> usize {
        2usize - 1usize
    }
    #[allow(dead_code)]
    pub fn get_capAllowWrite(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
    pub fn set_capAllowWrite(&mut self, capAllowWrite: u64) {
        self.0.set_bits(0usize..1usize, capAllowWrite)
    }
    #[allow(dead_code)]
    pub const fn width_of_capAllowWrite() -> usize {
        1usize - 0usize
    }
}
impl fmt::Debug for seL4_CapRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_CapRights_Unpacked {
    pub capAllowGrantReply: u64,
    pub capAllowGrant: u64,
    pub capAllowRead: u64,
    pub capAllowWrite: u64,
}
impl seL4_CapRights_Unpacked {
    pub fn pack(self) -> seL4_CapRights {
        match self {
            Self {
                capAllowGrantReply,
                capAllowGrant,
                capAllowRead,
                capAllowWrite,
            } => seL4_CapRights::new(
                capAllowGrantReply,
                capAllowGrant,
                capAllowRead,
                capAllowWrite,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_CNode_CapData(pub SeL4Bitfield<u64, 1usize>);
pub type seL4_CNode_CapData_t = seL4_CNode_CapData;
impl seL4_CNode_CapData {
    pub fn new(guard: u64, guardSize: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_guard(guard);
        this.set_guardSize(guardSize);
        this
    }
    pub fn unpack(&self) -> seL4_CNode_CapData_Unpacked {
        seL4_CNode_CapData_Unpacked {
            guard: self.get_guard(),
            guardSize: self.get_guardSize(),
        }
    }
    #[allow(dead_code)]
    pub fn get_guard(&self) -> u64 {
        self.0.get_bits(6usize..64usize)
    }
    pub fn set_guard(&mut self, guard: u64) {
        self.0.set_bits(6usize..64usize, guard)
    }
    #[allow(dead_code)]
    pub const fn width_of_guard() -> usize {
        64usize - 6usize
    }
    #[allow(dead_code)]
    pub fn get_guardSize(&self) -> u64 {
        self.0.get_bits(0usize..6usize)
    }
    pub fn set_guardSize(&mut self, guardSize: u64) {
        self.0.set_bits(0usize..6usize, guardSize)
    }
    #[allow(dead_code)]
    pub const fn width_of_guardSize() -> usize {
        6usize - 0usize
    }
}
impl fmt::Debug for seL4_CNode_CapData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_CNode_CapData_Unpacked {
    pub guard: u64,
    pub guardSize: u64,
}
impl seL4_CNode_CapData_Unpacked {
    pub fn pack(self) -> seL4_CNode_CapData {
        match self {
            Self { guard, guardSize } => seL4_CNode_CapData::new(guard, guardSize),
        }
    }
}
