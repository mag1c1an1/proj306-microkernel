use crate::define_bitfield_type;

pub mod cte;
pub mod mdb;


// pub mod zombie;

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct deriveCap_ret {
//     pub status: exception_t,
//     pub cap: cap_t,
// }
//
// #[repr(C)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct extra_caps_t {
//     pub excaprefs: [pptr_t; seL4_MsgMaxExtraCaps],
// }
//
// #[repr(C)]
// #[derive(Clone, Copy, Debug)]
// struct CNodeCapData {
//     pub words: [usize; 1],
// }
//
// impl CNodeCapData {
//     #[inline]
//     pub fn new(data: usize) -> Self {
//         CNodeCapData { words: [data] }
//     }
//
//     #[inline]
//     pub fn get_guard(&self) -> usize {
//         (self.words[0] & 0xffffffffffffffc0usize) >> 6
//     }
//
//     #[inline]
//     pub fn get_guard_size(&self) -> usize {
//         self.words[0] & 0x3fusize
//     }
// }
//
//
#[repr(usize)]
#[derive(Eq, PartialEq, Debug)]
pub enum CapType {
    Null = 0,
    Untyped = 2,
    Endpoint = 4,
    Notification = 6,
    Reply = 8,
    CNode = 10,
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
    CapASIDControl = 11,
    ASIDPool = 13,
    IOPort = 19,
    IOPortControl = 31,
}

// shift field_range offset signed
define_bitfield_type! {
   RawCap, 2, 59..64 => {
        new_null_cap, CapType::Null as usize => {},
        new_untyped_cap, CapType::Untyped as usize => {
            untyped_ptr, set_untyped_ptr, 0, 0..48, 0, true,
            block_size,  set_untyped_block_size, 0,64..70, 0, false,
            is_device, set_untyped_is_device, 0, 70..71, 0, false,
            free_index,  set_untyped_free_index, 0, 80..128, 0, false,
        },
        new_endpoint_cap, CapType::Endpoint as usize => {
            ep_ptr,  set_ep_ptr, 0, 0..48, 0, true,
            can_send,  set_ep_can_send, 0, 55..56, 0, false,
            can_receive,  set_ep_can_receive, 0, 56..57, 0, false,
            can_grant,  set_ep_can_grant, 0, 57..58, 0, false,
            can_grant_reply,  set_ep_can_grant_reply, 0, 58..59, 0, false,
            ep_badge,  set_ep_badge, 0, 64..128, 0, false,
        },
        new_notification_cap, CapType::Notification as usize => {
            ntfn_ptr,  set_nf_ptr, 0, 0..48, 0, true,
            ntfn_can_send,  set_nf_can_send, 0, 57..58, 0, false,
            ntfn_can_receive,  set_nf_can_receive, 0, 58..59, 0, false,
            ntfn_badge,  set_nf_badge, 0, 64..128, 0, false
        },
        new_reply_cap, CapType::Reply as usize => {
            reply_master,  set_reply_master, 0, 0..1, 0, false,
            reply_can_grant,  set_reply_can_grant, 0, 1..2, 0, false,
            reply_tcb_ptr,  set_reply_tcb_ptr, 0, 64..128, 0, false
        },
        new_cnode_cap, CapType::CNode as usize => {
            cnode_ptr, set_cnode_ptr, 1, 0..47, 1, true,
            cnode_radix, set_cnode_radix, 0, 47..53, 0, false,
            cnode_guard_size, set_cnode_guard_size, 0, 53..59, 0, false,
            cnode_guard, set_cnode_guard, 0, 64..128, 0, false,
       },
        new_thread_cap, CapType::Thread as usize => {
            thread_tcb_ptr,  set_thread_tcb_ptr, 0, 0..48, 0, true,
        },
        new_irq_control_cap, CapType::IrqControl as usize => {},
        new_irq_handler_cap, CapType::IrqHandler as usize => {
            irq, set_irq_handler, 0, 64..76, 0, false,
        },
        new_zombie_cap, CapType::Zombie as usize => {
            zombie_type,  set_zombie_type, 0, 0..7, 0, false,
            zombie_id,  set_zombie_id, 0, 64..128, 0, false,
        },
        new_domain_cap, CapType::Domain as usize => {},
        new_frame_cap, CapType::Frame as usize => {
            frame_is_device,  set_frame_is_device, 0, 4..5, 0, false,
            frame_vm_rights,  set_frame_vm_rights, 0, 5..7, 0, false,
            frame_mapped_address,  set_frame_mapped_address, 0, 7..55, 0, true,
            frame_map_type, set_frame_map_type, 0, 55..57, 0, false,
            frame_size,  set_frame_size, 0, 57..59, 0, false,
            frame_base_ptr,  set_frame_base_ptr, 0, 64..112, 0, true,
            frame_mapped_asid,  set_frame_mapped_asid, 0, 112..128, 0, false,
        },
        new_page_table_cap,CapType::PageTable as usize => {
            // only store 28 bits
            pt_mapped_address,  set_pt_mapped_address, 20, 1..29, 20, true,
            pt_is_mapped,  set_pt_is_mapped, 0, 49..50, 0, false,
            pt_base_ptr,  set_pt_base_ptr, 0, 64..112, 0, true,
            pt_mapped_asid,  set_pt_mapped_asid, 0, 112..124, 0, false,
        },
        new_page_directory_cap, CapType::PageDirectory as usize => {
            // store 19 bits
            pd_mapped_address,  set_pd_mapped_address, 29, 1..20, 29, true,
            pd_is_mapped,  set_pd_is_mapped, 0, 49..50, 0, false,
            pd_base_ptr,  set_pd_base_ptr, 0, 64..112, 0, true,
            pd_mapped_asid,  set_pd_mapped_asid, 0, 112..124, 0,false,
        },
        new_pdpt_cap, CapType::PDPT as usize => {
            // store 10 bits
            pdpt_mapped_address,  set_pdpt_mapped_address, 38, 10..20, 38, true,
            pdpt_is_mapped,  set_pdpt_is_mapped, 0, 58..59, 0, false,
            pdpt_base_ptr,  set_pdpt_base_ptr, 0, 64..112, 0, true,
            pdpt_mapped_asid,  set_pdpt_mapped_asid, 0,112..124, 0, false,
        },
        new_pml4_cap, CapType::PDPT as usize => {
            pml4_mapped_asid,  set_pml4_mapped_asid, 0, 0..12, 0, false,
            pml4_is_mapped,  set_pml4_is_mapped, 0, 58..59, 0, false,
            pml4_base_ptr,  set_pml4_base_ptr, 0, 64..128, 0, false,
        },
        new_asid_control_cap, CapType::CapASIDControl as usize => {},
        new_asid_pool_cap, CapType::ASIDPool as usize => {
            asid_pool,  set_asid_pool, 11, 0..37, 11, true,
            asid_base,  set_asid_base, 0, 47..59, 0, false,
        },
        new_io_port_cap, CapType::IOPort as usize => {
            io_port_last_port,  set_io_port_last_port, 0, 24..40, 0, false,
            io_port_first_port,  set_io_port_first_port, 0, 40..56, 0, false,
        },
        new_io_port_control_cap, CapType::IOPortControl as usize => {
        }
    }
}

mod test {
    use ktest::ktest;

    use crate::common::SeL4Bitfield;
    use crate::cspace::raw::{CapType, RawCap};

    #[ktest]
    fn cap_size_test() {
        assert_eq!(core::mem::size_of::<RawCap>(), 16);
    }

    #[ktest]
    fn null_cap_test() {
        let cap = RawCap::new_null_cap();
        assert_eq!(cap.typ(), CapType::Null as usize);
    }

    #[ktest]
    fn cnode_cap_test() {
        let ptr = 0xffff_8000_0412_ea90;
        let radix = 2;
        let guard_size = 3;
        let guard = 4;
        let cnode_cap = RawCap::new_cnode_cap(ptr, radix, guard_size, guard);
        assert_eq!((ptr >> 1 << 1), cnode_cap.cnode_ptr());
        assert_eq!(radix, cnode_cap.cnode_radix());
        assert_eq!(guard_size, cnode_cap.cnode_guard_size());
        assert_eq!(guard, cnode_cap.cnode_guard());
        assert_eq!(cnode_cap.typ(), CapType::CNode as usize);
    }

    #[ktest]
    fn untyped_cap_test() {}
}

// plus_define_bitfield! {
//     cap_t, 2, 0, 59, 5 => {
//         new_null_cap, CapType::Null as usize => {},
//     }
// }
//
// /// capability 的公用方法
// impl cap_t {
//     pub fn update_data(&self, preserve: bool, new_data: usize) -> Self {
//         if self.isArchCap() {
//             return self.clone();
//         }
//         match self.get_cap_type() {
//             CapTag::Endpoint => {
//                 if !preserve && self.get_ep_badge() == 0 {
//                     let mut new_cap = self.clone();
//                     new_cap.set_ep_badge(new_data);
//                     new_cap
//                 } else {
//                     cap_t::new_null_cap()
//                 }
//             }
//
//             CapTag::Notification => {
//                 if !preserve && self.get_nf_badge() == 0 {
//                     let mut new_cap = self.clone();
//                     new_cap.set_nf_badge(new_data);
//                     new_cap
//                 } else {
//                     cap_t::new_null_cap()
//                 }
//             }
//
//             CapTag::CNode => {
//                 let w = CNodeCapData::new(new_data);
//                 let guard_size = w.get_guard_size();
//                 if guard_size + self.get_cnode_radix() > seL4_WordBits as usize {
//                     return cap_t::new_null_cap();
//                 }
//                 let guard = w.get_guard() & mask!(guard_size);
//                 let mut new_cap = self.clone();
//                 new_cap.set_cnode_guard(guard);
//                 new_cap.set_cnode_guard_size(guard_size);
//                 new_cap
//             }
//             _ => self.clone(),
//         }
//     }
//
//     pub fn get_cap_type(&self) -> CapTag {
//         unsafe { core::mem::transmute::<u8, CapTag>(self.get_type() as u8) }
//     }
//
//     pub fn get_cap_ptr(&self) -> usize {
//         match self.get_cap_type() {
//             CapTag::Untyped => self.get_untyped_ptr(),
//             CapTag::Endpoint => self.get_ep_ptr(),
//             CapTag::Notification => self.get_nf_ptr(),
//             CapTag::CNode => self.get_cnode_ptr(),
//             CapTag::Thread => self.get_tcb_ptr(),
//             CapTag::Zombie => self.get_zombie_ptr(),
//             CapTag::Frame => self.get_frame_base_ptr(),
//             // CapTag::CapPageTableCap => self.get_pt_base_ptr(),
//             CapTag::ASIDPool => self.get_asid_pool(),
//             CapTag::PageTable => self.get_pt_base_ptr(),
//             CapTag::PageDirectory => self.get_pd_base_ptr(),
//             CapTag::PDPT => self.get_pdpt_base_ptr(),
//             CapTag::PML4 => self.get_pml4_base_ptr(),
//             _ => 0,
//         }
//     }
//
//     pub fn get_cap_size_bits(&self) -> usize {
//         match self.get_cap_type() {
//             CapTag::Untyped => self.get_untyped_block_size(),
//             CapTag::Endpoint => seL4_EndpointBits,
//             CapTag::Notification => seL4_NotificationBits,
//             CapTag::CNode => self.get_cnode_radix() + seL4_SlotBits,
//             CapTag::PageTable => PT_SIZE_BITS,
//             CapTag::Reply => seL4_ReplyBits,
//             _ => 0,
//         }
//     }
//
//     pub fn get_cap_is_physical(&self) -> bool {
//         match self.get_cap_type() {
//             CapTag::Untyped
//             | CapTag::Endpoint
//             | CapTag::Notification
//             | CapTag::CNode
//             | CapTag::Frame
//             | CapTag::ASIDPool
//             | CapTag::PageTable
//             | CapTag::Zombie
//             | CapTag::Thread => true,
//             _ => false,
//         }
//     }
//
//     pub fn isArchCap(&self) -> bool {
//         self.get_cap_type() as usize % 2 != 0
//     }
// }
//
// // vm related capability
// impl cap_t {
//     #[inline]
//     pub fn new_page_table_cap(
//         capPTIsMapped: usize,
//         capPTMappedAddress: usize,
//         capPTMappedASID: usize,
//         capPTBasePtr: usize,
//     ) -> Self {
//         let mut value = cap_t::default();
//         value.words[0] =
//             0 | (capPTIsMapped & 0x1) << 49 | (capPTMappedAddress & 0xfffffff00000) << 1;
//         value.words[1] = 0 | (capPTMappedASID & 0xfff) << 48 | (capPTBasePtr & 0xffffffffffff) >> 0;
//         value.words[0] |= ((CapTag::PageTable as usize & ((1usize << 5) - 1)) << 59);
//         value
//     }
//     #[inline]
//     pub fn get_pt_is_mapped(&self) -> usize {
//         let mask = ((1u128 << 1) - 1) as usize;
//         let mut ret = ((self.words[0] >> 49) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pt_is_mapped(&mut self, new_field: usize) {
//         let mask = ((1u128 << 1) - 1) as usize;
//         self.words[0] &= !(mask << 49);
//         self.words[0] |= (((new_field >> 0) & mask) << 49);
//     }
//     #[inline]
//     pub fn get_pt_mapped_address(&self) -> usize {
//         let mut ret = (self.words[0] & 0x1ffffffe00000) >> 1;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pt_mapped_address(&mut self, new_field: usize) {
//         self.words[0] &= !0x1ffffffe00000;
//         self.words[0] |= (new_field << 1) & 0x1ffffffe00000;
//     }
//     #[inline]
//     pub fn get_pt_mapped_asid(&self) -> usize {
//         let mut ret = (self.words[1] & 0xfff000000000000) >> 48;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pt_mapped_asid(&mut self, new_field: usize) {
//         self.words[1] &= !0xfff000000000000;
//         self.words[1] |= (new_field << 48) & 0xfff000000000000;
//     }
//     #[inline]
//     pub fn get_pt_base_ptr(&self) -> usize {
//         let mask = ((1u128 << 48) - 1) as usize;
//         let mut ret = ((self.words[1] >> 0) & mask) << 0;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pt_base_ptr(&mut self, new_field: usize) {
//         let mask = ((1u128 << 48) - 1) as usize;
//         self.words[1] &= !(mask << 0);
//         self.words[1] |= (((new_field >> 0) & mask) << 0);
//     }
//     #[inline]
//     pub fn new_page_directory_cap(
//         cap_pd_is_mapped: usize,
//         cap_pd_mapped_address: usize,
//         cap_pd_mapped_asid: usize,
//         cap_pd_base_ptr: usize,
//     ) -> Self {
//         let mut value = cap_t::default();
//         value.words[0] =
//             0 | (cap_pd_is_mapped & 0x1) << 49 | (cap_pd_mapped_address & 0xffffe0000000) << 1;
//         value.words[1] = 0 | (cap_pd_mapped_asid & 0xfff) << 48 | (cap_pd_base_ptr & 0xffffffffffff) >> 0;
//         value.words[0] |= ((CapTag::PageDirectory as usize & ((1usize << 5) - 1)) << 59);
//         value
//     }
//     #[inline]
//     pub fn get_pd_is_mapped(&self) -> usize {
//         let mask = ((1u128 << 1) - 1) as usize;
//         let mut ret = ((self.words[0] >> 49) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pd_is_mapped(&mut self, new_field: usize) {
//         let mask = ((1u128 << 1) - 1) as usize;
//         self.words[0] &= !(mask << 49);
//         self.words[0] |= (((new_field >> 0) & mask) << 49);
//     }
//     #[inline]
//     pub fn get_pd_mapped_address(&self) -> usize {
//         let mut ret = (self.words[0] & 0x1ffffc0000000) >> 1;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pd_mapped_address(&mut self, new_field: usize) {
//         let mask = ((1u128 << 19) - 1) as usize;
//         self.words[0] &= !0x1ffffc0000000;
//         self.words[0] |= (new_field << 1) & 0x1ffffc0000000;
//     }
//     #[inline]
//     pub fn get_pd_mapped_asid(&self) -> usize {
//         let mask = ((1u128 << 12) - 1) as usize;
//         let mut ret = ((self.words[1] >> 48) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pd_mapped_asid(&mut self, new_field: usize) {
//         let mask = ((1u128 << 12) - 1) as usize;
//         self.words[1] &= !(mask << 48);
//         self.words[1] |= (((new_field >> 0) & mask) << 48);
//     }
//     #[inline]
//     pub fn get_pd_base_ptr(&self) -> usize {
//         let mask = ((1u128 << 48) - 1) as usize;
//         let mut ret = ((self.words[1] >> 0) & mask) << 0;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pd_base_ptr(&mut self, new_field: usize) {
//         let mask = ((1u128 << 48) - 1) as usize;
//         self.words[1] &= !(mask << 0);
//         self.words[1] |= (((new_field >> 0) & mask) << 0);
//     }
//     #[inline]
//     pub fn new_pdpt_cap(
//         capPDPTIsMapped: usize,
//         capPDPTMappedAddress: usize,
//         capPDPTMappedASID: usize,
//         capPDPTBasePtr: usize,
//     ) -> Self {
//         let mut value = cap_t::default();
//         value.words[0] =
//             0 | (capPDPTIsMapped & 0x1) << 58 | (capPDPTMappedAddress & 0xffc000000000) << 10;
//         value.words[1] =
//             0 | (capPDPTMappedASID & 0xfff) << 48 | (capPDPTBasePtr & 0xffffffffffff) >> 0;
//         value.words[0] |= ((CapTag::PDPT as usize & ((1usize << 5) - 1)) << 59);
//         value
//     }
//     #[inline]
//     pub fn get_pdpt_is_mapped(&self) -> usize {
//         let mask = ((1u128 << 1) - 1) as usize;
//         let mut ret = ((self.words[0] >> 58) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pdpt_is_mapped(&mut self, new_field: usize) {
//         let mask = ((1u128 << 1) - 1) as usize;
//         self.words[0] &= !(mask << 58);
//         self.words[0] |= (((new_field >> 0) & mask) << 58);
//     }
//     #[inline]
//     pub fn get_pdpt_mapped_address(&self) -> usize {
//         let mut ret = (self.words[0] & 0x3ff000000000000) >> 10;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pdpt_mapped_address(&mut self, new_field: usize) {
//         self.words[0] &= !0x3ff000000000000;
//         self.words[0] |= (new_field << 10) & 0x3ff000000000000;
//     }
//     #[inline]
//     pub fn get_pdpt_mapped_asid(&self) -> usize {
//         let mask = ((1u128 << 12) - 1) as usize;
//         let mut ret = ((self.words[1] >> 48) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pdpt_mapped_asid(&mut self, new_field: usize) {
//         let mask = ((1u128 << 12) - 1) as usize;
//         self.words[1] &= !(mask << 48);
//         self.words[1] |= (((new_field >> 0) & mask) << 48);
//     }
//     #[inline]
//     pub fn get_pdpt_base_ptr(&self) -> usize {
//         let mask = ((1u128 << 48) - 1) as usize;
//         let mut ret = ((self.words[1] >> 0) & mask) << 0;
//         if true && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pdpt_base_ptr(&mut self, new_field: usize) {
//         let mask = ((1u128 << 48) - 1) as usize;
//         self.words[1] &= !(mask << 0);
//         self.words[1] |= (((new_field >> 0) & mask) << 0);
//     }
//     #[inline]
//     pub fn new_pml4_cap(
//         capPML4MappedASID: usize,
//         capPML4IsMapped: usize,
//         capPML4BasePtr: usize,
//     ) -> Self {
//         let mut value = cap_t::default();
//         value.words[0] = 0 | (capPML4MappedASID & 0xfff) << 0 | (capPML4IsMapped & 0x1) << 58;
//         value.words[1] = 0 | capPML4BasePtr << 0;
//         value.words[0] |= ((CapTag::PML4 as usize & ((1usize << 5) - 1)) << 59);
//         value
//     }
//     #[inline]
//     pub fn get_pml4_mapped_asid(&self) -> usize {
//         let mask = ((1u128 << 12) - 1) as usize;
//         let mut ret = ((self.words[0] >> 0) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pml4_mapped_asid(&mut self, new_field: usize) {
//         let mask = ((1u128 << 12) - 1) as usize;
//         self.words[0] &= !(mask << 0);
//         self.words[0] |= (((new_field >> 0) & mask) << 0);
//     }
//     #[inline]
//     pub fn get_pml4_is_mapped(&self) -> usize {
//         let mask = ((1u128 << 1) - 1) as usize;
//         let mut ret = ((self.words[0] >> 58) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pml4_is_mapped(&mut self, new_field: usize) {
//         let mask = ((1u128 << 1) - 1) as usize;
//         self.words[0] &= !(mask << 58);
//         self.words[0] |= (((new_field >> 0) & mask) << 58);
//     }
//     #[inline]
//     pub fn get_pml4_base_ptr(&self) -> usize {
//         let mask = ((1u128 << 64) - 1) as usize;
//         let mut ret = ((self.words[1] >> 0) & mask) << 0;
//         if false && (ret & (1usize << 47)) != 0 {
//             ret |= 0xffff000000000000;
//         }
//         ret
//     }
//     #[inline]
//     pub fn set_pml4_base_ptr(&mut self, new_field: usize) {
//         let mask = ((1u128 << 64) - 1) as usize;
//         self.words[1] &= !(mask << 0);
//         self.words[1] |= (((new_field >> 0) & mask) << 0);
//     }
// }
//
// pub fn same_region_as(cap1: &cap_t, cap2: &cap_t) -> bool {
//     match cap1.get_cap_type() {
//         CapTag::Untyped => {
//             if cap2.get_cap_is_physical() {
//                 let aBase = cap1.get_untyped_ptr();
//                 let bBase = cap2.get_cap_ptr();
//
//                 let aTop = aBase + MASK!(cap1.get_untyped_block_size());
//                 let bTop = bBase + MASK!(cap2.get_cap_size_bits());
//                 return (aBase <= bBase) && (bTop <= aTop) && (bBase <= bTop);
//             }
//
//             return false;
//         }
//         CapTag::Frame => {
//             if cap2.get_cap_type() == CapTag::Frame {
//                 let botA = cap1.get_frame_base_ptr();
//                 let botB = cap2.get_frame_base_ptr();
//                 let topA = botA + MASK!(pageBitsForSize(cap1.get_frame_size()));
//                 let topB = botB + MASK!(pageBitsForSize(cap2.get_frame_size()));
//                 return (botA <= botB) && (topA >= topB) && (botB <= topB);
//             }
//             false
//         }
//         CapTag::Endpoint
//         | CapTag::Notification
//         | CapTag::PageTable
//         | CapTag::ASIDPool
//         | CapTag::Thread => {
//             if cap2.get_cap_type() == cap1.get_cap_type() {
//                 return cap1.get_cap_ptr() == cap2.get_cap_ptr();
//             }
//             false
//         }
//         CapTag::CapASIDControl | CapTag::Domain => {
//             if cap2.get_cap_type() == cap1.get_cap_type() {
//                 return true;
//             }
//             false
//         }
//         CapTag::CNode => {
//             if cap2.get_cap_type() == CapTag::CNode {
//                 return (cap1.get_cnode_ptr() == cap2.get_cnode_ptr())
//                     && (cap1.get_cnode_radix() == cap2.get_cnode_radix());
//             }
//             false
//         }
//         CapTag::IrqControl => match cap2.get_cap_type() {
//             CapTag::IrqControl | CapTag::IrqHandler => true,
//             _ => false,
//         },
//         CapTag::IrqHandler => {
//             if cap2.get_cap_type() == CapTag::IrqHandler {
//                 return cap1.get_irq_handler() == cap2.get_irq_handler();
//             }
//             false
//         }
//         _ => {
//             return false;
//         }
//     }
// }
//
// /// 判断两个cap指向的内核对象是否是同一个内存区域
// pub fn same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
//     if cap1.get_cap_type() == CapTag::Untyped {
//         return false;
//     }
//     if cap1.get_cap_type() == CapTag::IrqControl
//         && cap2.get_cap_type() == CapTag::IrqHandler
//     {
//         return false;
//     }
//     if cap1.isArchCap() && cap2.isArchCap() {
//         return arch_same_object_as(cap1, cap2);
//     }
//     same_region_as(cap1, cap2)
// }
//
// fn arch_same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
//     if cap1.get_cap_type() == CapTag::Frame && cap2.get_cap_type() == CapTag::Frame {
//         return cap1.get_frame_base_ptr() == cap2.get_frame_base_ptr()
//             && cap1.get_frame_size() == cap2.get_frame_size()
//             && (cap1.get_frame_is_device() == 0) == (cap2.get_frame_is_device() == 0);
//     }
//     same_region_as(cap1, cap2)
// }
//
// pub fn is_cap_revocable(derived_cap: &cap_t, src_cap: &cap_t) -> bool {
//     if derived_cap.isArchCap() {
//         return false;
//     }
//
//     match derived_cap.get_cap_type() {
//         CapTag::Endpoint => {
//             assert_eq!(src_cap.get_cap_type(), CapTag::Endpoint);
//             return derived_cap.get_ep_badge() != src_cap.get_ep_badge();
//         }
//
//         CapTag::Notification => {
//             assert_eq!(src_cap.get_cap_type(), CapTag::Notification);
//             return derived_cap.get_nf_badge() != src_cap.get_nf_badge();
//         }
//
//         CapTag::IrqHandler => {
//             return src_cap.get_cap_type() == CapTag::IrqControl;
//         }
//
//         CapTag::Untyped => {
//             return true;
//         }
//
//         _ => false,
//     }
// }
//
// #[no_mangle]
// pub fn Arch_finaliseCap(cap: &cap_t, final_: bool) -> finaliseCap_ret {
//     let mut fc_ret = finaliseCap_ret::default();
//     unimplemented!()
//     // match capability.get_cap_type() {
//     //     CapTag::CapFrameCap => {
//     //         if capability.get_frame_mapped_asid() != 0 {
//     //             match unmapPage(
//     //                 capability.get_frame_size(),
//     //                 capability.get_frame_mapped_asid(),
//     //                 capability.get_frame_mapped_address(),
//     //                 capability.get_frame_base_ptr(),
//     //             ) {
//     //                 Err(lookup_fault) => unsafe { current_lookup_fault = lookup_fault },
//     //                 _ => {}
//     //             }
//     //         }
//     //     }
//
//     //     CapTag::CapPageTableCap => {
//     //         if final_ && capability.get_pt_is_mapped() != 0 {
//     //             let asid = capability.get_pt_mapped_asid();
//     //             let find_ret = find_vspace_for_asid(asid);
//     //             let pte = capability.get_pt_base_ptr();
//     //             if find_ret.status == exception_t::EXCEPTION_NONE
//     //                 && find_ret.vspace_root.unwrap() as usize == pte
//     //             {
//     //                 deleteASID(asid, pte as *mut pte_t);
//     //             } else {
//     //                 convert_to_mut_type_ref::<pte_t>(pte)
//     //                     .unmap_page_table(asid, capability.get_pt_mapped_address());
//     //             }
//     //             if let Some(lookup_fault) = find_ret.lookup_fault {
//     //                 unsafe {
//     //                     current_lookup_fault = lookup_fault;
//     //                 }
//     //             }
//     //         }
//     //     }
//
//     //     CapTag::CapASIDPoolCap => {
//     //         if final_ {
//     //             deleteASIDPool(capability.get_asid_base(), capability.get_asid_pool() as *mut asid_pool_t);
//     //         }
//     //     }
//     //     _ => {}
//     // }
//     // fc_ret.remainder = cap_t::new_null_cap();
//     // fc_ret.cleanupInfo = cap_t::new_null_cap();
//     // fc_ret
// }
//
// #[repr(C)]
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct finaliseSlot_ret {
//     pub status: exception_t,
//     pub success: bool,
//     pub cleanupInfo: cap_t,
// }
//
// impl Default for finaliseSlot_ret {
//     fn default() -> Self {
//         finaliseSlot_ret {
//             status: exception_t::EXCEPTION_NONE,
//             success: true,
//             cleanupInfo: cap_t::default(),
//         }
//     }
// }
//
// #[repr(C)]
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct finaliseCap_ret {
//     pub remainder: cap_t,
//     pub cleanupInfo: cap_t,
// }
//
// impl Default for finaliseCap_ret {
//     fn default() -> Self {
//         finaliseCap_ret {
//             remainder: cap_t::default(),
//             cleanupInfo: cap_t::default(),
//         }
//     }
// }
//
// #[no_mangle]
// pub fn finaliseCap(cap: &cap_t, _final: bool, _exposed: bool) -> finaliseCap_ret {
//     todo!()
//     // let mut fc_ret = finaliseCap_ret::default();
//
//     // if capability.isArchCap() {
//     //     return Arch_finaliseCap(capability, _final);
//     // }
//     // match capability.get_cap_type() {
//     //     CapTag::CapEndpointCap => {
//     //         if _final {
//     //             // cancelAllIPC(capability.get_ep_ptr() as *mut endpoint_t);
//     //             convert_to_mut_type_ref::<endpoint_t>(capability.get_ep_ptr()).cancel_all_ipc()
//     //         }
//     //         fc_ret.remainder = cap_t::new_null_cap();
//     //         fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //         return fc_ret;
//     //     }
//     //     CapTag::CapNotificationCap => {
//     //         if _final {
//     //             let ntfn = convert_to_mut_type_ref::<notification_t>(capability.get_nf_ptr());
//     //             ntfn.safe_unbind_tcb();
//     //             ntfn.cancel_call_signal();
//     //         }
//     //         fc_ret.remainder = cap_t::new_null_cap();
//     //         fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //         return fc_ret;
//     //     }
//     //     CapTag::CapReplyCap | CapTag::CapNullCap | CapTag::CapDomainCap => {
//     //         fc_ret.remainder = cap_t::new_null_cap();
//     //         fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //         return fc_ret;
//     //     }
//     //     _ => {
//     //         if _exposed {
//     //             panic!("finaliseCap: failed to finalise immediately.");
//     //         }
//     //     }
//     // }
//
//     // match capability.get_cap_type() {
//     //     CapTag::CapCNodeCap => {
//     //         return if _final {
//     //             fc_ret.remainder = Zombie_new(
//     //                 1usize << capability.get_cnode_radix(),
//     //                 capability.get_cnode_radix(),
//     //                 capability.get_cnode_ptr(),
//     //             );
//     //             fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //             fc_ret
//     //         } else {
//     //             fc_ret.remainder = cap_t::new_null_cap();
//     //             fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //             fc_ret
//     //         }
//     //     }
//     //     CapTag::CapThreadCap => {
//     //         if _final {
//     //             let tcb = convert_to_mut_type_ref::<tcb_t>(capability.get_tcb_ptr());
//     //             #[cfg(feature = "ENABLE_SMP")]
//     //             unsafe {
//     //                 crate::deps::remoteTCBStall(tcb)
//     //             };
//     //             let cte_ptr = tcb.get_cspace_mut_ref(tcbCTable);
//     //             safe_unbind_notification(tcb);
//     //             tcb.cancel_ipc();
//     //             tcb.suspend();
//     //             unsafe {
//     //                 // tcbDebugRemove(tcb as *mut tcb_t);
//     //                 tcb_debug_remove(tcb);
//     //             }
//     //             fc_ret.remainder =
//     //                 Zombie_new(tcbCNodeEntries, ZombieType_ZombieTCB, cte_ptr.get_ptr());
//     //             fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //             return fc_ret;
//     //         }
//     //     }
//     //     CapTag::CapZombieCap => {
//     //         fc_ret.remainder = capability.clone();
//     //         fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //         return fc_ret;
//     //     }
//     //     CapTag::CapIrqHandlerCap => {
//     //         if _final {
//     //             let irq = capability.get_irq_handler();
//     //             deletingIRQHandler(irq);
//     //             fc_ret.remainder = cap_t::new_null_cap();
//     //             fc_ret.cleanupInfo = capability.clone();
//     //             return fc_ret;
//     //         }
//     //     }
//     //     _ => {
//     //         fc_ret.remainder = cap_t::new_null_cap();
//     //         fc_ret.cleanupInfo = cap_t::new_null_cap();
//     //         return fc_ret;
//     //     }
//     // }
//     // fc_ret.remainder = cap_t::new_null_cap();
//     // fc_ret.cleanupInfo = cap_t::new_null_cap();
//     // return fc_ret;
// }
//
// #[no_mangle]
// pub fn post_cap_deletion(cap: &cap_t) {
//     if cap.get_cap_type() == CapTag::IrqHandler {
//         let irq = cap.get_irq_handler();
//         // setIRQState(IRQState::IRQInactive, irq);
//         unimplemented!()
//     }
// }
//
// #[no_mangle]
// pub fn preemptionPoint() -> exception_t {
//     unsafe {
//         unimplemented!()
//         // ksWorkUnitsCompleted += 1;
//         // if ksWorkUnitsCompleted >= CONFIG_MAX_NUM_WORK_UNITS_PER_PREEMPTION {
//         //     ksWorkUnitsCompleted = 0;
//
//         //     if isIRQPending() {
//         //         return exception_t::EXCEPTION_PREEMTED;
//         //     }
//         // }
//         // exception_t::EXCEPTION_NONE
//     }
// }
//
// // #[no_mangle]
// // fn deleteASID(asid: asid_t, vspace: *mut pte_t) {
// // unsafe {
// //     if let Err(lookup_fault) = delete_asid(
// //         asid,
// //         vspace,
// //         &get_currenct_thread().get_cspace(tcbVTable).capability,
// //     ) {
// //         current_lookup_fault = lookup_fault;
// //     }
// // }
// // }
//
// // #[no_mangle]
// // fn deleteASIDPool(asid_base: asid_t, pool: *mut asid_pool_t) {
// // unsafe {
// //     if let Err(lookup_fault) = delete_asid_pool(
// //         asid_base,
// //         pool,
// //         &get_currenct_thread().get_cspace(tcbVTable).capability,
// //     ) {
// //         current_lookup_fault = lookup_fault;
// //     }
// // }
// // }
//
// plus_define_bitfield! {
//     seL4_CapRights_t, 1, 0, 0, 0 => {
//         new, 0 => {
//             allow_grant_reply, get_allow_grant_reply, set_allow_grant_reply, 0, 3, 1, 0, false,
//             allow_grant, get_allow_grant, set_allow_grant, 0, 2, 1, 0, false,
//             allow_read, get_allow_read, set_allow_read, 0, 1, 1, 0, false,
//             allow_write, get_allow_write, set_allow_write, 0, 0, 1, 0, false
//         }
//     }
// }
//
// impl seL4_CapRights_t {
//     #[inline]
//     pub fn from_word(word: usize) -> Self {
//         Self { words: [word] }
//     }
// }
