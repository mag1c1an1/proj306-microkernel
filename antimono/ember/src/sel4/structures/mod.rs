#![allow(non_camel_case_types)]

pub mod cspace;
pub mod vspace;

use self::cspace::{cap_t, cte::cte_t};

use super::{
    seL4_MsgMaxExtraCaps, seL4_MsgMaxLength, CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS,
    MAX_NUM_FREEMEM_REG, MAX_NUM_RESV_REG,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_BootInfoHeader {
    pub id: usize,
    pub len: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_X86_BootInfo_mmap_t {
    pub header: seL4_BootInfoHeader,
    pub mmap_length: u32,
    pub mmap: [seL4_X86_mb_mmap_t; 50],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_X86_mb_mmap_t {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32,
}

/// kernel virtual memory region
#[derive(Copy, Clone)]
pub struct region_t {
    pub start: usize,
    pub end: usize,
}

/// physical virtual memory region
#[derive(Copy, Clone, Debug)]
pub struct p_region_t {
    pub start: usize,
    pub end: usize,
}

/// user virtual memory region
#[derive(Copy, Clone)]
pub struct v_region_t {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ui_info_t {}

pub type seL4_SlotPos = usize;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct seL4_SlotRegion {
    pub start: seL4_SlotPos,
    pub end: seL4_SlotPos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_UntypedDesc {
    pub paddr: usize,
    pub sizeBits: u8,
    pub isDevice: u8,
    pub padding: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_BootInfo {
    pub extraLen: usize,
    pub nodeID: usize,
    pub numNodes: usize,
    pub numIOPTLevels: usize,
    pub ipcBuffer: *const seL4_IPCBuffer,
    pub empty: seL4_SlotRegion,
    pub sharedFrames: seL4_SlotRegion,
    pub userImageFrames: seL4_SlotRegion,
    pub userImagePaging: seL4_SlotRegion,
    pub ioSpaceCaps: seL4_SlotRegion,
    pub extraBIPages: seL4_SlotRegion,
    pub initThreadCNodeSizeBits: usize,
    pub initThreadDomain: usize,
    pub untyped: seL4_SlotRegion,
    pub untypedList: [seL4_UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndks_boot_t {
    pub reserved: [p_region_t; MAX_NUM_RESV_REG],
    pub resv_count: usize,
    pub freemem: [region_t; MAX_NUM_FREEMEM_REG],
    pub bi_frame: *mut seL4_BootInfo,
    pub slot_pos_cur: seL4_SlotPos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rootserver_mem_t {
    pub cnode: usize,
    pub vspace: usize,
    pub asid_pool: usize,
    pub ipc_buf: usize,
    pub boot_info: usize,
    pub extra_bi: usize,
    pub tcb: usize,
    pub paging: region_t,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct create_frames_of_region_ret_t {
    pub region: seL4_SlotRegion,
    pub success: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct lookupCap_ret_t {
    pub status: exception_t,
    pub cap: cap_t,
}

impl Default for lookupCap_ret_t {
    fn default() -> Self {
        lookupCap_ret_t {
            status: exception_t::EXCEPTION_NONE,
            cap: cap_t::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct lookupCapAndSlot_ret_t {
    pub status: exception_t,
    pub cap: cap_t,
    pub slot: *mut cte_t,
}

impl Default for lookupCapAndSlot_ret_t {
    fn default() -> Self {
        lookupCapAndSlot_ret_t {
            status: exception_t::EXCEPTION_NONE,
            cap: cap_t::default(),
            slot: 0 as *mut cte_t,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct syscall_error_t {
    pub invalidArgumentNumber: usize,
    pub invalidCapNumber: usize,
    pub rangeErrorMin: usize,
    pub rangeErrorMax: usize,
    pub memoryLeft: usize,
    pub failedLookupWasSource: usize,
    pub _type: usize,
}

pub type pptr_t = usize;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum exception_t {
    EXCEPTION_NONE,
    EXCEPTION_FAULT,
    EXCEPTION_LOOKUP_FAULT,
    EXCEPTION_SYSCALL_ERROR,
    EXCEPTION_PREEMTED,
    padding = isize::MAX - 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_IPCBuffer {
    pub tag: usize,
    pub msg: [usize; seL4_MsgMaxLength],
    pub userData: usize,
    pub caps_or_badges: [usize; seL4_MsgMaxExtraCaps],
    pub receiveCNode: usize,
    pub receiveIndex: usize,
    pub receiveDepth: usize,
    pub uintrFlag: usize,
    pub async_cid: usize,
}

impl seL4_IPCBuffer {
    pub fn get_extra_cptr(&self, i: usize) -> usize {
        self.caps_or_badges[i]
    }
}
