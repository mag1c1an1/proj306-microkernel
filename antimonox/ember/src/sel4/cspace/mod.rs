use crate::{
    plus_define_bitfield,
    sel4::{
        seL4_EndpointBits, seL4_MsgMaxExtraCaps, seL4_NotificationBits, seL4_ReplyBits,
        seL4_SlotBits,
        utils::{convert_to_mut_type_ref, pageBitsForSize},
        wordBits, PT_SIZE_BITS,
    },
    MASK,
};

use super::{exception::exception_t, vspace::pptr_t};

pub mod cte;
pub mod mdb;
pub mod zombie;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct deriveCap_ret {
    pub status: exception_t,
    pub cap: cap_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct extra_caps_t {
    pub excaprefs: [pptr_t; seL4_MsgMaxExtraCaps],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct CNodeCapData {
    pub words: [usize; 1],
}

impl CNodeCapData {
    #[inline]
    pub fn new(data: usize) -> Self {
        CNodeCapData { words: [data] }
    }

    #[inline]
    pub fn get_guard(&self) -> usize {
        (self.words[0] & 0xffffffffffffffc0usize) >> 6
    }

    #[inline]
    pub fn get_guard_size(&self) -> usize {
        self.words[0] & 0x3fusize
    }
}

/// Cap 在内核态中的种类枚举
#[derive(Eq, PartialEq, Debug)]
pub enum CapTag {
    CapNullCap = 0,
    CapUntypedCap = 2,
    CapEndpointCap = 4,
    CapNotificationCap = 6,
    CapReplyCap = 8,
    CapCNodeCap = 10,
    CapThreadCap = 12,
    CapIrqControlCap = 14,
    CapIrqHandlerCap = 16,
    CapZombieCap = 18,
    CapDomainCap = 20,
    CapFrameCap = 1,
    CapPageTableCap = 3,
    CapPageDirectoryCap = 5,
    CapPDPTCap = 7,
    CapPML4Cap = 9,
    CapASIDControlCap = 11,
    CapASIDPoolCap = 13,
    CapIOPortCap = 19,
    CapIOPortControlCap = 31,
}

// cap_t 表示一个capability，由两个机器字组成，包含了类型、对象元数据以及指向内核对象的指针。
// 每个类型的capability的每个字段都实现了get和set方法。
plus_define_bitfield! {
    cap_t, 2, 0, 59, 5 => {
        new_null_cap, CapTag::CapNullCap as usize => {},
        new_untyped_cap, CapTag::CapUntypedCap as usize => {
            capPtr, get_untyped_ptr, set_untyped_ptr, 0, 0, 48, 0, true,
            capFreeIndex, get_untyped_free_index, set_untyped_free_index, 1, 16, 48, 0, false,
            capIsDevice, get_untyped_is_device, set_untyped_is_device, 1, 6, 1, 0, false,
            capBlockSize, get_untyped_block_size, set_untyped_block_size, 1, 0, 6, 0, false
        },
        new_endpoint_cap, CapTag::CapEndpointCap as usize => {
            capCanGrantReply, get_ep_can_grant_reply, set_ep_can_grant_reply, 0, 58, 1, 0, false,
            capCanGrant, get_ep_can_grant, set_ep_can_grant, 0, 57, 1, 0, false,
            capCanSend, get_ep_can_send, set_ep_can_send, 0, 55, 1, 0, false,
            capCanReceive, get_ep_can_receive, set_ep_can_receive, 0, 56, 1, 0, false,
            capEPPtr, get_ep_ptr, set_ep_ptr, 0, 0, 48, 0, true,
            capEPBadge, get_ep_badge, set_ep_badge, 1, 0, 64, 0, false
        },
        new_notification_cap, CapTag::CapNotificationCap as usize => {
            capNtfnCanReceive, get_nf_can_receive, set_nf_can_receive, 0, 58, 1, 0, false,
            capNtfnCanSend, get_nf_can_send, set_nf_can_send, 0, 57, 1, 0, false,
            capNtfnPtr, get_nf_ptr, set_nf_ptr, 0, 0, 48, 0, true,
            capNtfnBadge, get_nf_badge, set_nf_badge, 1, 0, 64, 0, false
        },
        new_reply_cap, CapTag::CapReplyCap as usize => {
            capReplyCanGrant, get_reply_can_grant, set_reply_can_grant, 0, 1, 1, 0, false,
            capReplyMaster, get_reply_master, set_reply_master, 0, 0, 1, 0, false,
            capTCBPtr, get_reply_tcb_ptr, set_reply_tcb_ptr, 1, 0, 64, 0, false
        },
        new_cnode_cap, CapTag::CapCNodeCap as usize => {
            capCNodeRadix, get_cnode_radix, set_cnode_radix, 0, 47, 6, 0, false,
            capCNodeGuardSize, get_cnode_guard_size, set_cnode_guard_size, 0, 53, 6, 0, false,
            capCNodePtr, get_cnode_ptr, set_cnode_ptr, 0, 0, 47, 1, true,
            capCNodeGuard, get_cnode_guard, set_cnode_guard, 1, 0, 64, 0, false
        },
        new_thread_cap, CapTag::CapThreadCap as usize => {
            capTCBPtr, get_tcb_ptr, set_tcb_ptr, 0, 0, 48, 0, true
        },
        new_irq_control_cap, CapTag::CapIrqControlCap as usize => {},
        new_irq_handler_cap, CapTag::CapIrqHandlerCap as usize => {
            capIRQ, get_irq_handler, set_irq_handler, 1, 0, 12, 0, false
        },
        new_zombie_cap, CapTag::CapZombieCap as usize => {
            capZombieType, get_zombie_type, set_zombie_type, 0, 0, 7, 0, false,
            capZombieID, get_zombie_id, set_zombie_id, 1, 0, 64, 0, false
        },
        new_domain_cap, CapTag::CapDomainCap as usize => {},
        new_frame_cap, CapTag::CapFrameCap as usize => {
            capFSize, get_frame_size, set_frame_size, 0, 57, 2, 0, false,
            capFMapType,get_frame_map_type, set_frame_map_type, 0, 55, 2, 0, false,
            capFMappedAddress, get_frame_mapped_address, set_frame_mapped_address, 0, 7, 48, 0, true,
            capFVMRights, get_frame_vm_rights, set_frame_vm_rights, 0, 5, 2, 0, false,
            capFIsDevice, get_frame_is_device, set_frame_is_device, 0, 4, 1, 0, false,
            capFMappedASID, get_frame_mapped_asid, set_frame_mapped_asid, 1, 48, 16, 0, false,
            capFBasePtr, get_frame_base_ptr, set_frame_base_ptr, 1, 0, 48, 0, true
        },
        new_asid_control_cap, CapTag::CapASIDControlCap as usize => {},
        new_asid_pool_cap, CapTag::CapASIDPoolCap as usize => {
            capASIDBase, get_asid_base, set_asid_base, 0, 43, 16, 0, false,
            capASIDPool, get_asid_pool, set_asid_pool, 0, 0, 37, 2, true
        },
        new_io_port_cap, CapTag::CapIOPortCap as usize => {
            capIOPortFirstPort, get_io_port_first_port, set_io_port_first_port, 0, 40, 20, 0, false,
            capIOPortLastPort, get_io_port_last_port, set_io_port_last_port, 0, 0, 20, 0, false
        },
        new_io_port_control_cap, CapTag::CapIOPortControlCap as usize => {
        }
    }
}

/// capability 的公用方法
impl cap_t {
    pub fn update_data(&self, preserve: bool, new_data: usize) -> Self {
        if self.isArchCap() {
            return self.clone();
        }
        match self.get_cap_type() {
            CapTag::CapEndpointCap => {
                if !preserve && self.get_ep_badge() == 0 {
                    let mut new_cap = self.clone();
                    new_cap.set_ep_badge(new_data);
                    new_cap
                } else {
                    cap_t::new_null_cap()
                }
            }

            CapTag::CapNotificationCap => {
                if !preserve && self.get_nf_badge() == 0 {
                    let mut new_cap = self.clone();
                    new_cap.set_nf_badge(new_data);
                    new_cap
                } else {
                    cap_t::new_null_cap()
                }
            }

            CapTag::CapCNodeCap => {
                let w = CNodeCapData::new(new_data);
                let guard_size = w.get_guard_size();
                if guard_size + self.get_cnode_radix() > wordBits {
                    return cap_t::new_null_cap();
                }
                let guard = w.get_guard() & MASK!(guard_size);
                let mut new_cap = self.clone();
                new_cap.set_cnode_guard(guard);
                new_cap.set_cnode_guard_size(guard_size);
                new_cap
            }
            _ => self.clone(),
        }
    }

    pub fn get_cap_type(&self) -> CapTag {
        unsafe { core::mem::transmute::<u8, CapTag>(self.get_type() as u8) }
    }

    pub fn get_cap_ptr(&self) -> usize {
        match self.get_cap_type() {
            CapTag::CapUntypedCap => self.get_untyped_ptr(),
            CapTag::CapEndpointCap => self.get_ep_ptr(),
            CapTag::CapNotificationCap => self.get_nf_ptr(),
            CapTag::CapCNodeCap => self.get_cnode_ptr(),
            CapTag::CapThreadCap => self.get_tcb_ptr(),
            CapTag::CapZombieCap => self.get_zombie_ptr(),
            CapTag::CapFrameCap => self.get_frame_base_ptr(),
            // CapTag::CapPageTableCap => self.get_pt_base_ptr(),
            CapTag::CapASIDPoolCap => self.get_asid_pool(),
            CapTag::CapPageTableCap => self.get_pt_base_ptr(),
            CapTag::CapPageDirectoryCap => self.get_pd_base_ptr(),
            CapTag::CapPDPTCap => self.get_pdpt_base_ptr(),
            CapTag::CapPML4Cap => self.get_pml4_base_ptr(),
            _ => 0,
        }
    }

    pub fn get_cap_size_bits(&self) -> usize {
        match self.get_cap_type() {
            CapTag::CapUntypedCap => self.get_untyped_block_size(),
            CapTag::CapEndpointCap => seL4_EndpointBits,
            CapTag::CapNotificationCap => seL4_NotificationBits,
            CapTag::CapCNodeCap => self.get_cnode_radix() + seL4_SlotBits,
            CapTag::CapPageTableCap => PT_SIZE_BITS,
            CapTag::CapReplyCap => seL4_ReplyBits,
            _ => 0,
        }
    }

    pub fn get_cap_is_physical(&self) -> bool {
        match self.get_cap_type() {
            CapTag::CapUntypedCap
            | CapTag::CapEndpointCap
            | CapTag::CapNotificationCap
            | CapTag::CapCNodeCap
            | CapTag::CapFrameCap
            | CapTag::CapASIDPoolCap
            | CapTag::CapPageTableCap
            | CapTag::CapZombieCap
            | CapTag::CapThreadCap => true,
            _ => false,
        }
    }

    pub fn isArchCap(&self) -> bool {
        self.get_cap_type() as usize % 2 != 0
    }
}

// vm related capability
impl cap_t {
    #[inline]
    pub fn new_page_table_cap(
        capPTIsMapped: usize,
        capPTMappedAddress: usize,
        capPTMappedASID: usize,
        capPTBasePtr: usize,
    ) -> Self {
        let mut value = cap_t::default();
        value.words[0] =
            0 | (capPTIsMapped & 0x1) << 49 | (capPTMappedAddress & 0xfffffff00000) << 1;
        value.words[1] = 0 | (capPTMappedASID & 0xfff) << 48 | (capPTBasePtr & 0xffffffffffff) >> 0;
        value.words[0] |= ((CapTag::CapPageTableCap as usize & ((1usize << 5) - 1)) << 59);
        value
    }
    #[inline]
    pub fn get_pt_is_mapped(&self) -> usize {
        let mask = ((1u128 << 1) - 1) as usize;
        let mut ret = ((self.words[0] >> 49) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pt_is_mapped(&mut self, new_field: usize) {
        let mask = ((1u128 << 1) - 1) as usize;
        self.words[0] &= !(mask << 49);
        self.words[0] |= (((new_field >> 0) & mask) << 49);
    }
    #[inline]
    pub fn get_pt_mapped_address(&self) -> usize {
        let mut ret = (self.words[0] & 0x1ffffffe00000) >> 1;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pt_mapped_address(&mut self, new_field: usize) {
        self.words[0] &= !0x1ffffffe00000;
        self.words[0] |= (new_field << 1) & 0x1ffffffe00000;
    }
    #[inline]
    pub fn get_pt_mapped_asid(&self) -> usize {
        let mut ret = (self.words[1] & 0xfff000000000000) >> 48;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pt_mapped_asid(&mut self, new_field: usize) {
        self.words[1] &= !0xfff000000000000;
        self.words[1] |= (new_field << 48) & 0xfff000000000000;
    }
    #[inline]
    pub fn get_pt_base_ptr(&self) -> usize {
        let mask = ((1u128 << 48) - 1) as usize;
        let mut ret = ((self.words[1] >> 0) & mask) << 0;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pt_base_ptr(&mut self, new_field: usize) {
        let mask = ((1u128 << 48) - 1) as usize;
        self.words[1] &= !(mask << 0);
        self.words[1] |= (((new_field >> 0) & mask) << 0);
    }
    #[inline]
    pub fn new_page_directory_cap(
        capPDIsMapped: usize,
        capPDMappedAddress: usize,
        capPDMappedASID: usize,
        capPDBasePtr: usize,
    ) -> Self {
        let mut value = cap_t::default();
        value.words[0] =
            0 | (capPDIsMapped & 0x1) << 49 | (capPDMappedAddress & 0xffffe0000000) << 1;
        value.words[1] = 0 | (capPDMappedASID & 0xfff) << 48 | (capPDBasePtr & 0xffffffffffff) >> 0;
        value.words[0] |= ((CapTag::CapPageDirectoryCap as usize & ((1usize << 5) - 1)) << 59);
        value
    }
    #[inline]
    pub fn get_pd_is_mapped(&self) -> usize {
        let mask = ((1u128 << 1) - 1) as usize;
        let mut ret = ((self.words[0] >> 49) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pd_is_mapped(&mut self, new_field: usize) {
        let mask = ((1u128 << 1) - 1) as usize;
        self.words[0] &= !(mask << 49);
        self.words[0] |= (((new_field >> 0) & mask) << 49);
    }
    #[inline]
    pub fn get_pd_mapped_address(&self) -> usize {
        let mut ret = (self.words[0] & 0x1ffffc0000000) >> 1;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pd_mapped_address(&mut self, new_field: usize) {
        let mask = ((1u128 << 19) - 1) as usize;
        self.words[0] &= !0x1ffffc0000000;
        self.words[0] |= (new_field << 1) & 0x1ffffc0000000;
    }
    #[inline]
    pub fn get_pd_mapped_asid(&self) -> usize {
        let mask = ((1u128 << 12) - 1) as usize;
        let mut ret = ((self.words[1] >> 48) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pd_mapped_asid(&mut self, new_field: usize) {
        let mask = ((1u128 << 12) - 1) as usize;
        self.words[1] &= !(mask << 48);
        self.words[1] |= (((new_field >> 0) & mask) << 48);
    }
    #[inline]
    pub fn get_pd_base_ptr(&self) -> usize {
        let mask = ((1u128 << 48) - 1) as usize;
        let mut ret = ((self.words[1] >> 0) & mask) << 0;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pd_base_ptr(&mut self, new_field: usize) {
        let mask = ((1u128 << 48) - 1) as usize;
        self.words[1] &= !(mask << 0);
        self.words[1] |= (((new_field >> 0) & mask) << 0);
    }
    #[inline]
    pub fn new_pdpt_cap(
        capPDPTIsMapped: usize,
        capPDPTMappedAddress: usize,
        capPDPTMappedASID: usize,
        capPDPTBasePtr: usize,
    ) -> Self {
        let mut value = cap_t::default();
        value.words[0] =
            0 | (capPDPTIsMapped & 0x1) << 58 | (capPDPTMappedAddress & 0xffc000000000) << 10;
        value.words[1] =
            0 | (capPDPTMappedASID & 0xfff) << 48 | (capPDPTBasePtr & 0xffffffffffff) >> 0;
        value.words[0] |= ((CapTag::CapPDPTCap as usize & ((1usize << 5) - 1)) << 59);
        value
    }
    #[inline]
    pub fn get_pdpt_is_mapped(&self) -> usize {
        let mask = ((1u128 << 1) - 1) as usize;
        let mut ret = ((self.words[0] >> 58) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pdpt_is_mapped(&mut self, new_field: usize) {
        let mask = ((1u128 << 1) - 1) as usize;
        self.words[0] &= !(mask << 58);
        self.words[0] |= (((new_field >> 0) & mask) << 58);
    }
    #[inline]
    pub fn get_pdpt_mapped_address(&self) -> usize {
        let mut ret = (self.words[0] & 0x3ff000000000000) >> 10;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pdpt_mapped_address(&mut self, new_field: usize) {
        self.words[0] &= !0x3ff000000000000;
        self.words[0] |= (new_field << 10) & 0x3ff000000000000;
    }
    #[inline]
    pub fn get_pdpt_mapped_asid(&self) -> usize {
        let mask = ((1u128 << 12) - 1) as usize;
        let mut ret = ((self.words[1] >> 48) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pdpt_mapped_asid(&mut self, new_field: usize) {
        let mask = ((1u128 << 12) - 1) as usize;
        self.words[1] &= !(mask << 48);
        self.words[1] |= (((new_field >> 0) & mask) << 48);
    }
    #[inline]
    pub fn get_pdpt_base_ptr(&self) -> usize {
        let mask = ((1u128 << 48) - 1) as usize;
        let mut ret = ((self.words[1] >> 0) & mask) << 0;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pdpt_base_ptr(&mut self, new_field: usize) {
        let mask = ((1u128 << 48) - 1) as usize;
        self.words[1] &= !(mask << 0);
        self.words[1] |= (((new_field >> 0) & mask) << 0);
    }
    #[inline]
    pub fn new_pml4_cap(
        capPML4MappedASID: usize,
        capPML4IsMapped: usize,
        capPML4BasePtr: usize,
    ) -> Self {
        let mut value = cap_t::default();
        value.words[0] = 0 | (capPML4MappedASID & 0xfff) << 0 | (capPML4IsMapped & 0x1) << 58;
        value.words[1] = 0 | capPML4BasePtr << 0;
        value.words[0] |= ((CapTag::CapPML4Cap as usize & ((1usize << 5) - 1)) << 59);
        value
    }
    #[inline]
    pub fn get_pml4_mapped_asid(&self) -> usize {
        let mask = ((1u128 << 12) - 1) as usize;
        let mut ret = ((self.words[0] >> 0) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pml4_mapped_asid(&mut self, new_field: usize) {
        let mask = ((1u128 << 12) - 1) as usize;
        self.words[0] &= !(mask << 0);
        self.words[0] |= (((new_field >> 0) & mask) << 0);
    }
    #[inline]
    pub fn get_pml4_is_mapped(&self) -> usize {
        let mask = ((1u128 << 1) - 1) as usize;
        let mut ret = ((self.words[0] >> 58) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pml4_is_mapped(&mut self, new_field: usize) {
        let mask = ((1u128 << 1) - 1) as usize;
        self.words[0] &= !(mask << 58);
        self.words[0] |= (((new_field >> 0) & mask) << 58);
    }
    #[inline]
    pub fn get_pml4_base_ptr(&self) -> usize {
        let mask = ((1u128 << 64) - 1) as usize;
        let mut ret = ((self.words[1] >> 0) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_pml4_base_ptr(&mut self, new_field: usize) {
        let mask = ((1u128 << 64) - 1) as usize;
        self.words[1] &= !(mask << 0);
        self.words[1] |= (((new_field >> 0) & mask) << 0);
    }
}

pub fn same_region_as(cap1: &cap_t, cap2: &cap_t) -> bool {
    match cap1.get_cap_type() {
        CapTag::CapUntypedCap => {
            if cap2.get_cap_is_physical() {
                let aBase = cap1.get_untyped_ptr();
                let bBase = cap2.get_cap_ptr();

                let aTop = aBase + MASK!(cap1.get_untyped_block_size());
                let bTop = bBase + MASK!(cap2.get_cap_size_bits());
                return (aBase <= bBase) && (bTop <= aTop) && (bBase <= bTop);
            }

            return false;
        }
        CapTag::CapFrameCap => {
            if cap2.get_cap_type() == CapTag::CapFrameCap {
                let botA = cap1.get_frame_base_ptr();
                let botB = cap2.get_frame_base_ptr();
                let topA = botA + MASK!(pageBitsForSize(cap1.get_frame_size()));
                let topB = botB + MASK!(pageBitsForSize(cap2.get_frame_size()));
                return (botA <= botB) && (topA >= topB) && (botB <= topB);
            }
            false
        }
        CapTag::CapEndpointCap
        | CapTag::CapNotificationCap
        | CapTag::CapPageTableCap
        | CapTag::CapASIDPoolCap
        | CapTag::CapThreadCap => {
            if cap2.get_cap_type() == cap1.get_cap_type() {
                return cap1.get_cap_ptr() == cap2.get_cap_ptr();
            }
            false
        }
        CapTag::CapASIDControlCap | CapTag::CapDomainCap => {
            if cap2.get_cap_type() == cap1.get_cap_type() {
                return true;
            }
            false
        }
        CapTag::CapCNodeCap => {
            if cap2.get_cap_type() == CapTag::CapCNodeCap {
                return (cap1.get_cnode_ptr() == cap2.get_cnode_ptr())
                    && (cap1.get_cnode_radix() == cap2.get_cnode_radix());
            }
            false
        }
        CapTag::CapIrqControlCap => match cap2.get_cap_type() {
            CapTag::CapIrqControlCap | CapTag::CapIrqHandlerCap => true,
            _ => false,
        },
        CapTag::CapIrqHandlerCap => {
            if cap2.get_cap_type() == CapTag::CapIrqHandlerCap {
                return cap1.get_irq_handler() == cap2.get_irq_handler();
            }
            false
        }
        _ => {
            return false;
        }
    }
}

/// 判断两个cap指向的内核对象是否是同一个内存区域
pub fn same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
    if cap1.get_cap_type() == CapTag::CapUntypedCap {
        return false;
    }
    if cap1.get_cap_type() == CapTag::CapIrqControlCap
        && cap2.get_cap_type() == CapTag::CapIrqHandlerCap
    {
        return false;
    }
    if cap1.isArchCap() && cap2.isArchCap() {
        return arch_same_object_as(cap1, cap2);
    }
    same_region_as(cap1, cap2)
}

fn arch_same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
    if cap1.get_cap_type() == CapTag::CapFrameCap && cap2.get_cap_type() == CapTag::CapFrameCap {
        return cap1.get_frame_base_ptr() == cap2.get_frame_base_ptr()
            && cap1.get_frame_size() == cap2.get_frame_size()
            && (cap1.get_frame_is_device() == 0) == (cap2.get_frame_is_device() == 0);
    }
    same_region_as(cap1, cap2)
}

pub fn is_cap_revocable(derived_cap: &cap_t, src_cap: &cap_t) -> bool {
    if derived_cap.isArchCap() {
        return false;
    }

    match derived_cap.get_cap_type() {
        CapTag::CapEndpointCap => {
            assert_eq!(src_cap.get_cap_type(), CapTag::CapEndpointCap);
            return derived_cap.get_ep_badge() != src_cap.get_ep_badge();
        }

        CapTag::CapNotificationCap => {
            assert_eq!(src_cap.get_cap_type(), CapTag::CapNotificationCap);
            return derived_cap.get_nf_badge() != src_cap.get_nf_badge();
        }

        CapTag::CapIrqHandlerCap => {
            return src_cap.get_cap_type() == CapTag::CapIrqControlCap;
        }

        CapTag::CapUntypedCap => {
            return true;
        }

        _ => false,
    }
}

#[no_mangle]
pub fn Arch_finaliseCap(cap: &cap_t, final_: bool) -> finaliseCap_ret {
    let mut fc_ret = finaliseCap_ret::default();
    unimplemented!()
    // match capability.get_cap_type() {
    //     CapTag::CapFrameCap => {
    //         if capability.get_frame_mapped_asid() != 0 {
    //             match unmapPage(
    //                 capability.get_frame_size(),
    //                 capability.get_frame_mapped_asid(),
    //                 capability.get_frame_mapped_address(),
    //                 capability.get_frame_base_ptr(),
    //             ) {
    //                 Err(lookup_fault) => unsafe { current_lookup_fault = lookup_fault },
    //                 _ => {}
    //             }
    //         }
    //     }

    //     CapTag::CapPageTableCap => {
    //         if final_ && capability.get_pt_is_mapped() != 0 {
    //             let asid = capability.get_pt_mapped_asid();
    //             let find_ret = find_vspace_for_asid(asid);
    //             let pte = capability.get_pt_base_ptr();
    //             if find_ret.status == exception_t::EXCEPTION_NONE
    //                 && find_ret.vspace_root.unwrap() as usize == pte
    //             {
    //                 deleteASID(asid, pte as *mut pte_t);
    //             } else {
    //                 convert_to_mut_type_ref::<pte_t>(pte)
    //                     .unmap_page_table(asid, capability.get_pt_mapped_address());
    //             }
    //             if let Some(lookup_fault) = find_ret.lookup_fault {
    //                 unsafe {
    //                     current_lookup_fault = lookup_fault;
    //                 }
    //             }
    //         }
    //     }

    //     CapTag::CapASIDPoolCap => {
    //         if final_ {
    //             deleteASIDPool(capability.get_asid_base(), capability.get_asid_pool() as *mut asid_pool_t);
    //         }
    //     }
    //     _ => {}
    // }
    // fc_ret.remainder = cap_t::new_null_cap();
    // fc_ret.cleanupInfo = cap_t::new_null_cap();
    // fc_ret
}
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct finaliseSlot_ret {
    pub status: exception_t,
    pub success: bool,
    pub cleanupInfo: cap_t,
}

impl Default for finaliseSlot_ret {
    fn default() -> Self {
        finaliseSlot_ret {
            status: exception_t::EXCEPTION_NONE,
            success: true,
            cleanupInfo: cap_t::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct finaliseCap_ret {
    pub remainder: cap_t,
    pub cleanupInfo: cap_t,
}

impl Default for finaliseCap_ret {
    fn default() -> Self {
        finaliseCap_ret {
            remainder: cap_t::default(),
            cleanupInfo: cap_t::default(),
        }
    }
}
#[no_mangle]
pub fn finaliseCap(cap: &cap_t, _final: bool, _exposed: bool) -> finaliseCap_ret {
    todo!()
    // let mut fc_ret = finaliseCap_ret::default();

    // if capability.isArchCap() {
    //     return Arch_finaliseCap(capability, _final);
    // }
    // match capability.get_cap_type() {
    //     CapTag::CapEndpointCap => {
    //         if _final {
    //             // cancelAllIPC(capability.get_ep_ptr() as *mut endpoint_t);
    //             convert_to_mut_type_ref::<endpoint_t>(capability.get_ep_ptr()).cancel_all_ipc()
    //         }
    //         fc_ret.remainder = cap_t::new_null_cap();
    //         fc_ret.cleanupInfo = cap_t::new_null_cap();
    //         return fc_ret;
    //     }
    //     CapTag::CapNotificationCap => {
    //         if _final {
    //             let ntfn = convert_to_mut_type_ref::<notification_t>(capability.get_nf_ptr());
    //             ntfn.safe_unbind_tcb();
    //             ntfn.cancel_call_signal();
    //         }
    //         fc_ret.remainder = cap_t::new_null_cap();
    //         fc_ret.cleanupInfo = cap_t::new_null_cap();
    //         return fc_ret;
    //     }
    //     CapTag::CapReplyCap | CapTag::CapNullCap | CapTag::CapDomainCap => {
    //         fc_ret.remainder = cap_t::new_null_cap();
    //         fc_ret.cleanupInfo = cap_t::new_null_cap();
    //         return fc_ret;
    //     }
    //     _ => {
    //         if _exposed {
    //             panic!("finaliseCap: failed to finalise immediately.");
    //         }
    //     }
    // }

    // match capability.get_cap_type() {
    //     CapTag::CapCNodeCap => {
    //         return if _final {
    //             fc_ret.remainder = Zombie_new(
    //                 1usize << capability.get_cnode_radix(),
    //                 capability.get_cnode_radix(),
    //                 capability.get_cnode_ptr(),
    //             );
    //             fc_ret.cleanupInfo = cap_t::new_null_cap();
    //             fc_ret
    //         } else {
    //             fc_ret.remainder = cap_t::new_null_cap();
    //             fc_ret.cleanupInfo = cap_t::new_null_cap();
    //             fc_ret
    //         }
    //     }
    //     CapTag::CapThreadCap => {
    //         if _final {
    //             let tcb = convert_to_mut_type_ref::<tcb_t>(capability.get_tcb_ptr());
    //             #[cfg(feature = "ENABLE_SMP")]
    //             unsafe {
    //                 crate::deps::remoteTCBStall(tcb)
    //             };
    //             let cte_ptr = tcb.get_cspace_mut_ref(tcbCTable);
    //             safe_unbind_notification(tcb);
    //             tcb.cancel_ipc();
    //             tcb.suspend();
    //             unsafe {
    //                 // tcbDebugRemove(tcb as *mut tcb_t);
    //                 tcb_debug_remove(tcb);
    //             }
    //             fc_ret.remainder =
    //                 Zombie_new(tcbCNodeEntries, ZombieType_ZombieTCB, cte_ptr.get_ptr());
    //             fc_ret.cleanupInfo = cap_t::new_null_cap();
    //             return fc_ret;
    //         }
    //     }
    //     CapTag::CapZombieCap => {
    //         fc_ret.remainder = capability.clone();
    //         fc_ret.cleanupInfo = cap_t::new_null_cap();
    //         return fc_ret;
    //     }
    //     CapTag::CapIrqHandlerCap => {
    //         if _final {
    //             let irq = capability.get_irq_handler();
    //             deletingIRQHandler(irq);
    //             fc_ret.remainder = cap_t::new_null_cap();
    //             fc_ret.cleanupInfo = capability.clone();
    //             return fc_ret;
    //         }
    //     }
    //     _ => {
    //         fc_ret.remainder = cap_t::new_null_cap();
    //         fc_ret.cleanupInfo = cap_t::new_null_cap();
    //         return fc_ret;
    //     }
    // }
    // fc_ret.remainder = cap_t::new_null_cap();
    // fc_ret.cleanupInfo = cap_t::new_null_cap();
    // return fc_ret;
}

#[no_mangle]
pub fn post_cap_deletion(cap: &cap_t) {
    if cap.get_cap_type() == CapTag::CapIrqHandlerCap {
        let irq = cap.get_irq_handler();
        // setIRQState(IRQState::IRQInactive, irq);
        unimplemented!()
    }
}

#[no_mangle]
pub fn preemptionPoint() -> exception_t {
    unsafe {
        unimplemented!()
        // ksWorkUnitsCompleted += 1;
        // if ksWorkUnitsCompleted >= CONFIG_MAX_NUM_WORK_UNITS_PER_PREEMPTION {
        //     ksWorkUnitsCompleted = 0;

        //     if isIRQPending() {
        //         return exception_t::EXCEPTION_PREEMTED;
        //     }
        // }
        // exception_t::EXCEPTION_NONE
    }
}

// #[no_mangle]
// fn deleteASID(asid: asid_t, vspace: *mut pte_t) {
// unsafe {
//     if let Err(lookup_fault) = delete_asid(
//         asid,
//         vspace,
//         &get_currenct_thread().get_cspace(tcbVTable).capability,
//     ) {
//         current_lookup_fault = lookup_fault;
//     }
// }
// }

// #[no_mangle]
// fn deleteASIDPool(asid_base: asid_t, pool: *mut asid_pool_t) {
// unsafe {
//     if let Err(lookup_fault) = delete_asid_pool(
//         asid_base,
//         pool,
//         &get_currenct_thread().get_cspace(tcbVTable).capability,
//     ) {
//         current_lookup_fault = lookup_fault;
//     }
// }
// }

plus_define_bitfield! {
    seL4_CapRights_t, 1, 0, 0, 0 => {
        new, 0 => {
            allow_grant_reply, get_allow_grant_reply, set_allow_grant_reply, 0, 3, 1, 0, false,
            allow_grant, get_allow_grant, set_allow_grant, 0, 2, 1, 0, false,
            allow_read, get_allow_read, set_allow_read, 0, 1, 1, 0, false,
            allow_write, get_allow_write, set_allow_write, 0, 0, 1, 0, false
        }
    }
}

impl seL4_CapRights_t {
    #[inline]
    pub fn from_word(word: usize) -> Self {
        Self { words: [word] }
    }
}
